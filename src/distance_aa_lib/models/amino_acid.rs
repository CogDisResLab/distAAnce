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

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::string::ToString;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AminoAcid {
    name: String,
    short_name: String,
    abbreviation: String,
    side_chain: String,
    molecular_weight: f64,
    codon: Vec<String>,
}

impl AminoAcid {
    #[must_use]
    pub fn new(
        name: &str,
        short_name: &str,
        abbreviation: &str,
        side_chain: &str,
        molecular_weight: f64,
        codon: &[&str],
    ) -> Self {
        Self {
            name: name.to_string(),
            short_name: short_name.to_string(),
            abbreviation: abbreviation.to_string(),
            side_chain: side_chain.to_string(),
            molecular_weight,
            codon: codon.iter().map(ToString::to_string).collect(),
        }
    }
    #[must_use]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    #[must_use]
    pub fn get_short_name(&self) -> String {
        self.short_name.clone()
    }
    #[must_use]
    pub fn get_abbreviation(&self) -> String {
        self.abbreviation.clone()
    }
    #[must_use]
    pub fn get_side_chain(&self) -> String {
        self.side_chain.clone()
    }
    #[must_use]
    pub const fn get_molecular_weight(&self) -> f64 {
        self.molecular_weight
    }
    #[must_use]
    pub fn get_codon(&self) -> Vec<String> {
        self.codon.clone()
    }
    #[must_use]
    pub fn get_codon_string(&self) -> String {
        self.codon.join(", ")
    }
    #[must_use]
    pub fn get_codon_count(&self) -> usize {
        self.codon.len()
    }
}

impl Display for AminoAcid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}\tShort Name: {}\tAbbreviation: {}\tSide Chain: {}\tMolecular Weight: {}\tCodon: {}",
            self.name,
            self.short_name,
            self.abbreviation,
            self.side_chain,
            self.molecular_weight,
            self.codon.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCU", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_name(), "Alanine");
    }
    #[test]
    fn test_get_short_name() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCU", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_short_name(), "Ala");
    }
    #[test]
    fn test_get_abbreviation() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCU", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_abbreviation(), "A");
    }
    #[test]
    fn test_get_side_chain() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCU", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_side_chain(), "Nonpolar");
    }
    #[test]
    fn test_get_molecular_weight() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCU", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_molecular_weight(), 89.09);
    }
    #[test]
    fn test_get_codon() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCT", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_codon(), vec!["GCT", "GCC", "GCA", "GCG"]);
    }
    #[test]
    fn test_get_codon_string() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCT", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_codon_string(), "GCT, GCC, GCA, GCG");
    }
    #[test]
    fn test_get_codon_count() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCT", "GCC", "GCA", "GCG"],
        );
        assert_eq!(amino_acid.get_codon_count(), 4);
    }

    #[test]
    fn test_fmt() {
        let amino_acid = AminoAcid::new(
            "Alanine",
            "Ala",
            "A",
            "Nonpolar",
            89.09,
            &["GCT", "GCC", "GCA", "GCG"],
        );
        assert_eq!(
            format!("{}", amino_acid),
            "Name: Alanine\tShort Name: Ala\tAbbreviation: A\tSide Chain: Nonpolar\tMolecular Weight: 89.09\tCodon: GCT, GCC, GCA, GCG"
        );
    }
}
