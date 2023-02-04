use super::vector_3f::Vector3F;

pub struct Matrix44F {
    pub matrix: [f64; 16]
}

impl Matrix44F {
    const ROW_LENGTH: usize = 4;
    const COLUMN_LENGTH: usize = 4;
    const MATRIX_LENGTH: usize = Matrix44F::ROW_LENGTH * Matrix44F::COLUMN_LENGTH;

    pub fn default() -> Matrix44F {
        return Matrix44F { matrix: [0.0; Matrix44F::MATRIX_LENGTH] };
    }

    pub fn new_from_value(value: f64) -> Matrix44F {
        return Matrix44F { matrix: [value; Matrix44F::MATRIX_LENGTH] };
    }

    pub fn new(matrix: [f64; 16]) -> Matrix44F {
        return Matrix44F { matrix: (matrix) };
    }

    pub fn get_row(self, row_index: usize) -> [f64; 4] {
        if row_index < Matrix44F::MATRIX_LENGTH / Matrix44F::ROW_LENGTH {
            let index = Matrix44F::ROW_LENGTH * row_index;
            return [self.matrix[index], self.matrix[index + 1], self.matrix[index + 2], self.matrix[index + 3]];
        }
        return [0.0, 0.0, 0.0, 0.0];
    }

    pub fn get_col(self, col_index: usize) -> [f64; 4] {
        if col_index < Matrix44F::MATRIX_LENGTH / Matrix44F::COLUMN_LENGTH {
            return [self.matrix[col_index], self.matrix[col_index + (Matrix44F::ROW_LENGTH * 1)], self.matrix[col_index + (Matrix44F::ROW_LENGTH * 2)], self.matrix[col_index + (Matrix44F::ROW_LENGTH * 3)]];
        }
        return [0.0; 4];
    }

    pub fn get_at(self, row_index: usize, col_index: usize) -> f64 {
        if row_index * Matrix44F::ROW_LENGTH + col_index < Matrix44F::MATRIX_LENGTH {
            return self.matrix[row_index * Matrix44F::ROW_LENGTH + col_index];
        }
        return 0.0;
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        let swap = self.matrix[i1];
        self.matrix[i1] = self.matrix[i2];
        self.matrix[i2] = swap;
    }

    pub fn transpose(&mut self) {
        self.swap(1, 4);
        self.swap(2, 8);
        self.swap(4, 12);
        self.swap(6, 9);
        self.swap(7, 13);
        self.swap(11, 14);
    }

    pub fn load_identity(&mut self) {
        self.matrix[0] = 1.0;
        self.matrix[5] = 1.0;
        self.matrix[10] = 1.0;
        self.matrix[15] = 1.0;
        self.matrix[1] = 0.0;
        self.matrix[2] = 0.0;
        self.matrix[3] = 0.0;
        self.matrix[4] = 0.0;
        self.matrix[6] = 0.0;
        self.matrix[7] = 0.0;
        self.matrix[8] = 0.0;
        self.matrix[9] = 0.0;
        self.matrix[11] = 0.0;
        self.matrix[12] = 0.0;
        self.matrix[13] = 0.0;
        self.matrix[14] = 0.0;
    }

    pub fn load_orthogonal(&mut self, right: f64, bottom: f64) {
        self.matrix[0] = 2.0 / right;
        self.matrix[5] = 2.0 / -bottom;
        self.matrix[10] = -1.0;
        self.matrix[12] = -1.0;
        self.matrix[13] = 1.0;
        self.matrix[15] = 1.0;
        self.matrix[1] = 0.0;
        self.matrix[2] = 0.0;
        self.matrix[3] = 0.0;
        self.matrix[4] = 0.0;
        self.matrix[6] = 0.0;
        self.matrix[7] = 0.0;
        self.matrix[8] = 0.0;
        self.matrix[9] = 0.0;
        self.matrix[11] = 0.0;
        self.matrix[14] = 0.0;
    }

    pub fn load_perspective(&mut self, right: f64, top: f64, near: f64, far: f64) {
        self.matrix[0] = near / right;
        self.matrix[5] = near / top;
        self.matrix[10] = -(far + near) / (far - near);
        self.matrix[11] = -1.0;
        self.matrix[14] = (-2.0 * far * near) / (far - near);
        self.matrix[1] = 0.0;
        self.matrix[2] = 0.0;
        self.matrix[3] = 0.0;
        self.matrix[4] = 0.0;
        self.matrix[6] = 0.0;
        self.matrix[7] = 0.0;
        self.matrix[8] = 0.0;
        self.matrix[9] = 0.0;
        self.matrix[12] = 0.0;
        self.matrix[13] = 0.0;
        self.matrix[15] = 0.0;
    }

    pub fn load_rotate_x(&mut self, angle: f64) {
        self.matrix[0] = 1.0;
        self.matrix[1] = 0.0;
        self.matrix[2] = 0.0;
        self.matrix[4] = 0.0;
        self.matrix[8] = 0.0;
        self.matrix[5] = angle.cos();
        self.matrix[6] = -(angle.sin());
        self.matrix[9] = angle.sin();
        self.matrix[10] = angle.cos();
    }

    pub fn load_rotate_y(&mut self, angle: f64) {
        self.matrix[0] = angle.cos();
        self.matrix[1] = 0.0;
        self.matrix[4] = 0.0;
        self.matrix[6] = 0.0;
        self.matrix[9] = 0.0;
        self.matrix[2] = angle.sin();
        self.matrix[5] = 1.0;
        self.matrix[8] = -(angle.sin());
        self.matrix[10] = angle.cos();
    }

    pub fn load_rotate_z(&mut self, angle: f64) {
        self.matrix[0] = angle.cos();
        self.matrix[1] = -(angle.sin());
        self.matrix[2] = 0.0;
        self.matrix[6] = 0.0;
        self.matrix[8] = 0.0;
        self.matrix[9] = 0.0;
        self.matrix[4] = angle.sin();
        self.matrix[5] = angle.cos();
        self.matrix[10] = 1.0;
    }

    pub fn load_rotate(&mut self, vector: Vector3F, angle: f64) {
        let v = Vector3F::get_normalized(vector);
        let c = angle.cos();
        let s = angle.sin();
        let ic = 1.0 - c; //inverse de c (qui est le cos de l'angle)

        //https://fr.wikipedia.org/wiki/Matrice_de_rotation#Axe_de_rotation
        self.matrix[0] = v.x * v.x * ic + c;
        self.matrix[5] = v.y * v.y * ic + c;
        self.matrix[10] = v.z * v.z * ic + c;

        self.matrix[1] = v.x * v.y * ic + v.z * s;
        self.matrix[4] = v.x * v.y * ic - v.z * s;

        self.matrix[2] = v.x * v.z * ic + v.y * s;
        self.matrix[8] = v.x * v.z * ic - v.y * s;

        self.matrix[6] = v.y * v.z * ic - v.x * s;
        self.matrix[9] = v.y * v.z * ic + v.x * s;

        // mettre le reste a zéro
        self.matrix[3] = 0.0;
        self.matrix[7]= 0.0;
        self.matrix[11] = 0.0;
        self.matrix[12] = 0.0;
        self.matrix[13] = 0.0;
        self.matrix[14] = 0.0;
        self.matrix[15] = 0.0;
    }

    pub fn load_view(&mut self, front: Vector3F, side: Vector3F, top: Vector3F) {
        self.matrix[0] = side.x;
        self.matrix[4] = side.y;
        self.matrix[8] = side.z;

        self.matrix[1] = top.x;
        self.matrix[5] = top.y;
        self.matrix[9] = top.z;

        self.matrix[2] = -front.x;
        self.matrix[6] = -front.y;
        self.matrix[10] = -front.z;

        self.matrix[15] = 1.0;
        self.matrix[12] = 0.0;
        self.matrix[13] = 0.0;
        self.matrix[14] = 0.0;
        self.matrix[3] = 0.0;
        self.matrix[7] = 0.0;
        self.matrix[11] = 0.0;
    }

    pub fn load_scale(&mut self, x: f64, y: f64, z: f64) {
        self.matrix[0] = x;
        self.matrix[5] = y;
        self.matrix[10] = z;
        self.matrix[15] = 1.0;
        self.matrix[1] = 0.0;
        self.matrix[2] = 0.0;
        self.matrix[3] = 0.0;
        self.matrix[4] = 0.0;
        self.matrix[6] = 0.0;
        self.matrix[7] = 0.0;
        self.matrix[8] = 0.0;
        self.matrix[9] = 0.0;
        self.matrix[11] = 0.0;
        self.matrix[12] = 0.0;
        self.matrix[13] = 0.0;
        self.matrix[14] = 0.0;
    }
}

impl std::ops::Mul<Vector3F> for Matrix44F {
    type Output = Vector3F;

    fn mul(self, v: Vector3F) -> Self::Output {
        return Vector3F {
            x: self.matrix[0] * v.x + self.matrix[1] * v.y + self.matrix[2] * v.z,
            y: self.matrix[4] * v.x + self.matrix[5] * v.y + self.matrix[6] * v.z,
            z: self.matrix[8] * v.x + self.matrix[9] * v.y + self.matrix[10] * v.z
        };
    }
}

impl std::ops::Mul<f64> for Matrix44F {
    type Output = Matrix44F;

    fn mul(self, d: f64) -> Self::Output {
        let mut new_matrix = Matrix44F::default();
        for i in 0..Matrix44F::MATRIX_LENGTH {
            new_matrix.matrix[i] = self.matrix[i] * d;
        }
        return new_matrix;
    }
}

impl std::ops::Mul<Matrix44F> for Matrix44F {
    type Output = Matrix44F;

    fn mul(self, other_matrix: Matrix44F) -> Self::Output {
        let mut new_matrix = Matrix44F::default();
        for i in 0..Matrix44F::ROW_LENGTH {
            for j in 0..Matrix44F::COLUMN_LENGTH {
                for k in 0..Matrix44F::ROW_LENGTH {
                    new_matrix.matrix[i * Matrix44F::ROW_LENGTH + j] += self.matrix[i * Matrix44F::ROW_LENGTH + k] * other_matrix.matrix[k * Matrix44F::ROW_LENGTH + j];
                }
            }
        }
        return new_matrix;
    }
}

impl Clone for Matrix44F {
    fn clone(&self) -> Self {
        return Matrix44F {
            matrix: self.matrix
        };
    }
}

impl Copy for Matrix44F {}

impl std::fmt::Display for Matrix44F {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut value: String = String::new();
        for i in 0..4 {
            value.push_str("[ ");
            for j in 0..4 {
                value.push_str(&self.matrix[i * 4 + j].to_string());
                value.push_str(" ");
            }
            value.push_str("]\n");
        }
        return write!(f, "{}", value);
    }
}