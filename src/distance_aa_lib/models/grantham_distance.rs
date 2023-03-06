// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2022 Cognitive Disorders Research Laboratory
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// *     http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

use crate::models::AminoAcid;
use fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GranthamDistance {
    first: AminoAcid,
    second: AminoAcid,
    distance: usize,
}

impl GranthamDistance {
    #[must_use]
    pub fn new(first: AminoAcid, second: AminoAcid, distance: usize) -> Self {
        Self {
            first,
            second,
            distance,
        }
    }
    #[must_use]
    pub fn get_first(&self) -> AminoAcid {
        self.first.clone()
    }
    #[must_use]
    pub fn get_second(&self) -> AminoAcid {
        self.second.clone()
    }
    #[must_use]
    pub const fn get_distance(&self) -> usize {
        self.distance
    }
}

impl Default for GranthamDistance {
    fn default() -> Self {
        Self {
            first: AminoAcid::default(),
            second: AminoAcid::default(),
            distance: 0,
        }
    }
}

impl Display for GranthamDistance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GranthamDistance {{ first: {}, second: {}, distance: {} }}",
            self.first, self.second, self.distance
        )
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grantham_distance =
            GranthamDistance::new(AminoAcid::from("A"), AminoAcid::from("C"), 2);
        assert_eq!(grantham_distance.get_first(), AminoAcid::from("A"));
        assert_eq!(grantham_distance.get_second(), AminoAcid::from("C"));
        assert_eq!(grantham_distance.get_distance(), 2);
    }

    #[test]
    fn test_default() {
        let grantham_distance = GranthamDistance::default();
        assert_eq!(grantham_distance.get_first(), AminoAcid::default());
        assert_eq!(grantham_distance.get_second(), AminoAcid::default());
        assert_eq!(grantham_distance.get_distance(), 0);
    }
}
