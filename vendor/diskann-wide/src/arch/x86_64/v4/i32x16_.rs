/*
 * Copyright (c) Microsoft Corporation.
 * Licensed under the MIT license.
 */

// x86 intrinsics
use std::arch::x86_64::*;

use crate::{
    BitMask,
    arch::x86_64::{
        V4,
        common::AllOnes,
        macros::{self, X86Default, X86LoadStore, X86Splat},
        v4::{i8x64_::i8x64, i16x32_::i16x32, i32x8_::i32x8, u8x64_::u8x64}, // direct import for Miri compat
    },
    constant::Const,
    helpers,
    traits::{SIMDAbs, SIMDDotProduct, SIMDMask, SIMDMulAdd, SIMDSelect, SIMDSumTree, SIMDVector},
};

/////
///// 32-bit signed
/////

macros::x86_define_register!(i32x16, __m512i, BitMask<16, V4>, i32, 16, V4);
macros::x86_define_splat!(i32x16, _mm512_set1_epi32, "avx512f");
macros::x86_define_default!(i32x16, _mm512_setzero_si512, "avx512f");
macros::x86_splitjoin!(__m512i, i32x16, i32x8);

helpers::unsafe_map_binary_op!(i32x16, std::ops::Add, add, _mm512_add_epi32, "avx512f");
helpers::unsafe_map_binary_op!(i32x16, std::ops::Sub, sub, _mm512_sub_epi32, "avx512f");
helpers::unsafe_map_binary_op!(i32x16, std::ops::Mul, mul, _mm512_mullo_epi32, "avx512f");
helpers::unsafe_map_unary_op!(i32x16, SIMDAbs, abs_simd, _mm512_abs_epi32, "avx512f");

helpers::unsafe_map_binary_op!(
    i32x16,
    std::ops::BitAnd,
    bitand,
    _mm512_and_si512,
    "avx512f"
);
helpers::unsafe_map_binary_op!(i32x16, std::ops::BitOr, bitor, _mm512_or_si512, "avx512f");
helpers::unsafe_map_binary_op!(
    i32x16,
    std::ops::BitXor,
    bitxor,
    _mm512_xor_si512,
    "avx512f"
);
helpers::unsafe_map_binary_op!(i32x16, std::ops::Shr, shr, _mm512_srav_epi32, "avx512f");
helpers::unsafe_map_binary_op!(i32x16, std::ops::Shl, shl, _mm512_sllv_epi32, "avx512f");
helpers::scalar_shift_by_splat!(i32x16, i32);

impl std::ops::Not for i32x16 {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        self ^ Self::from_underlying(self.arch(), <Self as SIMDVector>::Underlying::all_ones())
    }
}

impl SIMDMulAdd for i32x16 {
    #[inline(always)]
    fn mul_add_simd(self, rhs: Self, accumulator: Self) -> Self {
        self * rhs + accumulator
    }
}

macros::x86_avx512_load_store!(
    i32x16,
    _mm512_loadu_epi32,
    _mm512_maskz_loadu_epi32,
    _mm512_storeu_epi32,
    _mm512_mask_storeu_epi32,
    i32,
    "avx512f"
);

macros::x86_avx512_int_comparisons!(i32x16, _mm512_cmp_epi32_mask, "avx512f");

impl SIMDSumTree for i32x16 {
    #[inline(always)]
    fn sum_tree(self) -> i32 {
        // SAFETY: `_mm512_reduce_add_epi32` requires AVX512F - implied by V4
        unsafe { _mm512_reduce_add_epi32(self.0) }
    }
}

impl SIMDSelect<i32x16> for BitMask<16, V4> {
    #[inline(always)]
    fn select(self, x: i32x16, y: i32x16) -> i32x16 {
        // SAFETY: `_mm512_mask_blend_epi32` requires AVX512F - implied by V4
        i32x16::from_underlying(self.arch(), unsafe {
            _mm512_mask_blend_epi32(self.to_underlying(), y.to_underlying(), x.to_underlying())
        })
    }
}

//--------------//
// Dot Products //
//--------------//

impl SIMDDotProduct<i16x32> for i32x16 {
    #[inline(always)]
    fn dot_simd(self, left: i16x32, right: i16x32) -> Self {
        // Non-VNNI fallback: madd gives pairwise i16*i16→i32 sums, then add to accumulator.
        // Requires avx512bw (implied by V4). Avoids _mm512_dpwssd_epi32 (LLVM 21 bug).
        let r = unsafe {
            _mm512_add_epi32(self.0, _mm512_madd_epi16(left.0, right.0))
        };
        Self::from_underlying(self.arch(), r)
    }
}

impl SIMDDotProduct<u8x64, i8x64> for i32x16 {
    #[inline(always)]
    fn dot_simd(self, left: u8x64, right: i8x64) -> Self {
        // Non-VNNI fallback for _mm512_dpbusd_epi32.
        // u8 × i8 products fit in i16 (max |255×127|=32385 < 32767).
        // Process low/high 32-byte halves separately, convert to i16, multiply, sum pairs.
        let r = unsafe {
            let a_lo = _mm512_extracti64x4_epi64(left.0, 0);
            let a_hi = _mm512_extracti64x4_epi64(left.0, 1);
            let b_lo = _mm512_extracti64x4_epi64(right.0, 0);
            let b_hi = _mm512_extracti64x4_epi64(right.0, 1);

            let a_lo_16 = _mm512_cvtepu8_epi16(a_lo);
            let a_hi_16 = _mm512_cvtepu8_epi16(a_hi);
            let b_lo_16 = _mm512_cvtepi8_epi16(b_lo);
            let b_hi_16 = _mm512_cvtepi8_epi16(b_hi);

            let ones = _mm512_set1_epi16(1);
            // sum_lo[j] = a[2j]*b[2j] + a[2j+1]*b[2j+1], covering bytes 0-31 → lanes 0-7
            let sum_lo = _mm512_madd_epi16(_mm512_mullo_epi16(a_lo_16, b_lo_16), ones);
            // sum_hi[j] covers bytes 32-63 → lanes 8-15
            let sum_hi = _mm512_madd_epi16(_mm512_mullo_epi16(a_hi_16, b_hi_16), ones);

            // Reduce pairs within each 256-bit half using hadd, then fix ordering.
            let perm_idx = _mm256_set_epi32(7, 6, 3, 2, 5, 4, 1, 0);

            let sum_lo_lo = _mm512_extracti64x4_epi64(sum_lo, 0);
            let sum_lo_hi = _mm512_extracti64x4_epi64(sum_lo, 1);
            // hadd gives [lane0,lane1,lane4,lane5, lane2,lane3,lane6,lane7]
            let lanes_0_7 = _mm256_permutevar8x32_epi32(
                _mm256_hadd_epi32(sum_lo_lo, sum_lo_hi),
                perm_idx,
            );

            let sum_hi_lo = _mm512_extracti64x4_epi64(sum_hi, 0);
            let sum_hi_hi = _mm512_extracti64x4_epi64(sum_hi, 1);
            let lanes_8_15 = _mm256_permutevar8x32_epi32(
                _mm256_hadd_epi32(sum_hi_lo, sum_hi_hi),
                perm_idx,
            );

            let dot = _mm512_inserti64x4(_mm512_castsi256_si512(lanes_0_7), lanes_8_15, 1);
            _mm512_add_epi32(self.0, dot)
        };
        Self::from_underlying(self.arch(), r)
    }
}

impl SIMDDotProduct<i8x64, u8x64> for i32x16 {
    #[inline(always)]
    fn dot_simd(self, left: i8x64, right: u8x64) -> Self {
        self.dot_simd(right, left)
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod test_x86_i32 {
    use super::*;
    use crate::{reference::ReferenceScalarOps, test_utils};

    #[test]
    fn miri_test_load() {
        if let Some(arch) = V4::new_checked_uncached() {
            test_utils::test_load_simd::<i32, 16, i32x16>(arch);
        }
    }

    #[test]
    fn miri_test_store() {
        if let Some(arch) = V4::new_checked_uncached() {
            test_utils::test_store_simd::<i32, 16, i32x16>(arch);
        }
    }

    // constructors
    #[test]
    fn test_constructors() {
        if let Some(arch) = V4::new_checked_uncached() {
            test_utils::ops::test_splat::<i32, 16, i32x16>(arch);
        }
    }

    // Binary Ops
    test_utils::ops::test_add!(i32x16, 0xa93d54e3aab5d406, V4::new_checked_uncached());
    test_utils::ops::test_sub!(i32x16, 0x4b86c88f6958d930, V4::new_checked_uncached());
    test_utils::ops::test_mul!(i32x16, 0x0ad0524dc17b747a, V4::new_checked_uncached());
    test_utils::ops::test_fma!(i32x16, 0x277aca15e0552388, V4::new_checked_uncached());
    test_utils::ops::test_abs!(i32x16, 0x62ca26a68c1a238d, V4::new_checked_uncached());

    test_utils::ops::test_cmp!(i32x16, 0xdc88c2a44d17c78a, V4::new_checked_uncached());
    test_utils::ops::test_splitjoin!(i32x16 => i32x8, 0x475a19e80c2f3977, V4::new_checked_uncached());
    test_utils::ops::test_select!(i32x16, 0x568c7fcbf00fb03a, V4::new_checked_uncached());

    // Bit ops
    test_utils::ops::test_bitops!(i32x16, 0xc5f7d8d8df0b7b6c, V4::new_checked_uncached());

    // Dot Products
    test_utils::dot_product::test_dot_product!(
        (i16x32, i16x32) => i32x16, 0x145f89b446c03ff1, V4::new_checked_uncached()
    );
    test_utils::dot_product::test_dot_product!(
        (u8x64, i8x64) => i32x16, 0xa56e0de8ce99136c, V4::new_checked_uncached()
    );
    test_utils::dot_product::test_dot_product!(
        (i8x64, u8x64) => i32x16, 0xbcbcff932237df6d, V4::new_checked_uncached()
    );

    // Reductions
    test_utils::ops::test_sumtree!(i32x16, 0xe533708e69ca0117, V4::new_checked_uncached());
}
