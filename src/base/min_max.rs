use crate::storage::Storage;
use crate::{ComplexField, Dim, Matrix, Scalar, SimdComplexField, SimdPartialOrd, Vector};
use rand_distr::num_traits::{Signed, Zero};
use simba::simd::SimdSigned;

/// # Find the min and max components
impl<N: Scalar, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    /// Returns the absolute value of the component with the largest absolute value.
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// assert_eq!(Vector3::new(-1.0, 2.0, 3.0).amax(), 3.0);
    /// assert_eq!(Vector3::new(-1.0, -2.0, -3.0).amax(), 3.0);
    /// ```
    #[inline]
    pub fn amax(&self) -> N
    where
        N: Zero + SimdSigned + SimdPartialOrd,
    {
        self.fold_with(
            |e| e.unwrap_or(&N::zero()).simd_abs(),
            |a, b| a.simd_max(b.simd_abs()),
        )
    }

    /// Returns the the 1-norm of the complex component with the largest 1-norm.
    /// # Example
    /// ```
    /// # use nalgebra::{Vector3, Complex};
    /// assert_eq!(Vector3::new(
    ///     Complex::new(-3.0, -2.0),
    ///     Complex::new(1.0, 2.0),
    ///     Complex::new(1.0, 3.0)).camax(), 5.0);
    /// ```
    #[inline]
    pub fn camax(&self) -> N::SimdRealField
    where
        N: SimdComplexField,
    {
        self.fold_with(
            |e| e.unwrap_or(&N::zero()).simd_norm1(),
            |a, b| a.simd_max(b.simd_norm1()),
        )
    }

    /// Returns the component with the largest value.
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// assert_eq!(Vector3::new(-1.0, 2.0, 3.0).max(), 3.0);
    /// assert_eq!(Vector3::new(-1.0, -2.0, -3.0).max(), -1.0);
    /// assert_eq!(Vector3::new(5u32, 2, 3).max(), 5);
    /// ```
    #[inline]
    pub fn max(&self) -> N
    where
        N: SimdPartialOrd + Zero,
    {
        self.fold_with(
            |e| e.map(|e| e.inlined_clone()).unwrap_or(N::zero()),
            |a, b| a.simd_max(b.inlined_clone()),
        )
    }

    /// Returns the absolute value of the component with the smallest absolute value.
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// assert_eq!(Vector3::new(-1.0, 2.0, -3.0).amin(), 1.0);
    /// assert_eq!(Vector3::new(10.0, 2.0, 30.0).amin(), 2.0);
    /// ```
    #[inline]
    pub fn amin(&self) -> N
    where
        N: Zero + SimdPartialOrd + SimdSigned,
    {
        self.fold_with(
            |e| e.map(|e| e.simd_abs()).unwrap_or(N::zero()),
            |a, b| a.simd_min(b.simd_abs()),
        )
    }

    /// Returns the the 1-norm of the complex component with the smallest 1-norm.
    /// # Example
    /// ```
    /// # use nalgebra::{Vector3, Complex};
    /// assert_eq!(Vector3::new(
    ///     Complex::new(-3.0, -2.0),
    ///     Complex::new(1.0, 2.0),
    ///     Complex::new(1.0, 3.0)).camin(), 3.0);
    /// ```
    #[inline]
    pub fn camin(&self) -> N::SimdRealField
    where
        N: SimdComplexField,
    {
        self.fold_with(
            |e| {
                e.map(|e| e.simd_norm1())
                    .unwrap_or(N::SimdRealField::zero())
            },
            |a, b| a.simd_min(b.simd_norm1()),
        )
    }

    /// Returns the component with the smallest value.
    /// # Example
    /// ```
    /// # use nalgebra::Vector3;
    /// assert_eq!(Vector3::new(-1.0, 2.0, 3.0).min(), -1.0);
    /// assert_eq!(Vector3::new(1.0, 2.0, 3.0).min(), 1.0);
    /// assert_eq!(Vector3::new(5u32, 2, 3).min(), 2);
    /// ```
    #[inline]
    pub fn min(&self) -> N
    where
        N: SimdPartialOrd + Zero,
    {
        self.fold_with(
            |e| e.map(|e| e.inlined_clone()).unwrap_or(N::zero()),
            |a, b| a.simd_min(b.inlined_clone()),
        )
    }

    /// Computes the index of the matrix component with the largest absolute value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # extern crate num_complex;
    /// # extern crate nalgebra;
    /// # use num_complex::Complex;
    /// # use nalgebra::Matrix2x3;
    /// let mat = Matrix2x3::new(Complex::new(11.0, 1.0), Complex::new(-12.0, 2.0), Complex::new(13.0, 3.0),
    ///                          Complex::new(21.0, 43.0), Complex::new(22.0, 5.0), Complex::new(-23.0, 0.0));
    /// assert_eq!(mat.icamax_full(), (1, 0));
    /// ```
    #[inline]
    pub fn icamax_full(&self) -> (usize, usize)
    where
        N: ComplexField,
    {
        assert!(!self.is_empty(), "The input matrix must not be empty.");

        let mut the_max = unsafe { self.get_unchecked((0, 0)).norm1() };
        let mut the_ij = (0, 0);

        for j in 0..self.ncols() {
            for i in 0..self.nrows() {
                let val = unsafe { self.get_unchecked((i, j)).norm1() };

                if val > the_max {
                    the_max = val;
                    the_ij = (i, j);
                }
            }
        }

        the_ij
    }
}

impl<N: Scalar + PartialOrd + Signed, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    /// Computes the index of the matrix component with the largest absolute value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Matrix2x3;
    /// let mat = Matrix2x3::new(11, -12, 13,
    ///                          21, 22, -23);
    /// assert_eq!(mat.iamax_full(), (1, 2));
    /// ```
    #[inline]
    pub fn iamax_full(&self) -> (usize, usize) {
        assert!(!self.is_empty(), "The input matrix must not be empty.");

        let mut the_max = unsafe { self.get_unchecked((0, 0)).abs() };
        let mut the_ij = (0, 0);

        for j in 0..self.ncols() {
            for i in 0..self.nrows() {
                let val = unsafe { self.get_unchecked((i, j)).abs() };

                if val > the_max {
                    the_max = val;
                    the_ij = (i, j);
                }
            }
        }

        the_ij
    }
}

// TODO: find a way to avoid code duplication just for complex number support.
/// # Find the min and max components (vector-specific methods)
impl<N: Scalar, D: Dim, S: Storage<N, D>> Vector<N, D, S> {
    /// Computes the index of the vector component with the largest complex or real absolute value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # extern crate num_complex;
    /// # extern crate nalgebra;
    /// # use num_complex::Complex;
    /// # use nalgebra::Vector3;
    /// let vec = Vector3::new(Complex::new(11.0, 3.0), Complex::new(-15.0, 0.0), Complex::new(13.0, 5.0));
    /// assert_eq!(vec.icamax(), 2);
    /// ```
    #[inline]
    pub fn icamax(&self) -> usize
    where
        N: ComplexField,
    {
        assert!(!self.is_empty(), "The input vector must not be empty.");

        let mut the_max = unsafe { self.vget_unchecked(0).norm1() };
        let mut the_i = 0;

        for i in 1..self.nrows() {
            let val = unsafe { self.vget_unchecked(i).norm1() };

            if val > the_max {
                the_max = val;
                the_i = i;
            }
        }

        the_i
    }

    /// Computes the index and value of the vector component with the largest value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Vector3;
    /// let vec = Vector3::new(11, -15, 13);
    /// assert_eq!(vec.argmax(), (2, 13));
    /// ```
    #[inline]
    pub fn argmax(&self) -> (usize, N)
    where
        N: PartialOrd,
    {
        assert!(!self.is_empty(), "The input vector must not be empty.");

        let mut the_max = unsafe { self.vget_unchecked(0) };
        let mut the_i = 0;

        for i in 1..self.nrows() {
            let val = unsafe { self.vget_unchecked(i) };

            if val > the_max {
                the_max = val;
                the_i = i;
            }
        }

        (the_i, the_max.inlined_clone())
    }

    /// Computes the index of the vector component with the largest value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Vector3;
    /// let vec = Vector3::new(11, -15, 13);
    /// assert_eq!(vec.imax(), 2);
    /// ```
    #[inline]
    pub fn imax(&self) -> usize
    where
        N: PartialOrd,
    {
        self.argmax().0
    }

    /// Computes the index of the vector component with the largest absolute value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Vector3;
    /// let vec = Vector3::new(11, -15, 13);
    /// assert_eq!(vec.iamax(), 1);
    /// ```
    #[inline]
    pub fn iamax(&self) -> usize
    where
        N: PartialOrd + Signed,
    {
        assert!(!self.is_empty(), "The input vector must not be empty.");

        let mut the_max = unsafe { self.vget_unchecked(0).abs() };
        let mut the_i = 0;

        for i in 1..self.nrows() {
            let val = unsafe { self.vget_unchecked(i).abs() };

            if val > the_max {
                the_max = val;
                the_i = i;
            }
        }

        the_i
    }

    /// Computes the index and value of the vector component with the smallest value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Vector3;
    /// let vec = Vector3::new(11, -15, 13);
    /// assert_eq!(vec.argmin(), (1, -15));
    /// ```
    #[inline]
    pub fn argmin(&self) -> (usize, N)
    where
        N: PartialOrd,
    {
        assert!(!self.is_empty(), "The input vector must not be empty.");

        let mut the_min = unsafe { self.vget_unchecked(0) };
        let mut the_i = 0;

        for i in 1..self.nrows() {
            let val = unsafe { self.vget_unchecked(i) };

            if val < the_min {
                the_min = val;
                the_i = i;
            }
        }

        (the_i, the_min.inlined_clone())
    }

    /// Computes the index of the vector component with the smallest value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Vector3;
    /// let vec = Vector3::new(11, -15, 13);
    /// assert_eq!(vec.imin(), 1);
    /// ```
    #[inline]
    pub fn imin(&self) -> usize
    where
        N: PartialOrd,
    {
        self.argmin().0
    }

    /// Computes the index of the vector component with the smallest absolute value.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::Vector3;
    /// let vec = Vector3::new(11, -15, 13);
    /// assert_eq!(vec.iamin(), 0);
    /// ```
    #[inline]
    pub fn iamin(&self) -> usize
    where
        N: PartialOrd + Signed,
    {
        assert!(!self.is_empty(), "The input vector must not be empty.");

        let mut the_min = unsafe { self.vget_unchecked(0).abs() };
        let mut the_i = 0;

        for i in 1..self.nrows() {
            let val = unsafe { self.vget_unchecked(i).abs() };

            if val < the_min {
                the_min = val;
                the_i = i;
            }
        }

        the_i
    }
}
