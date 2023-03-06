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

#[allow(dead_code)]
impl AminoAcid {
    pub fn new(name: &str) -> Self {
        match name {
            "Alanine" => Self {
                name: "Alanine".to_string(),
                short_name: "Ala".to_string(),
                abbreviation: "A".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 89.09,
                codon: vec!["GCT", "GCC", "GCA", "GCG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Arginine" => Self {
                name: "Arginine".to_string(),
                short_name: "Arg".to_string(),
                abbreviation: "R".to_string(),
                side_chain: "Basic".to_string(),
                molecular_weight: 174.20,
                codon: vec!["CGT", "CGC", "CGA", "CGG", "AGA", "AGG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Asparagine" => Self {
                name: "Asparagine".to_string(),
                short_name: "Asn".to_string(),
                abbreviation: "N".to_string(),
                side_chain: "Polar".to_string(),
                molecular_weight: 132.12,
                codon: vec!["AAT", "AAC"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Aspartic Acid" => Self {
                name: "Aspartic Acid".to_string(),
                short_name: "Asp".to_string(),
                abbreviation: "Asp".to_string(),
                side_chain: "Acidic".to_string(),
                molecular_weight: 133.10,
                codon: vec!["GAU", "GAC"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Cysteine" => Self {
                name: "Cysteine".to_string(),
                short_name: "Cys".to_string(),
                abbreviation: "D".to_string(),
                side_chain: "Polar".to_string(),
                molecular_weight: 121.15,
                codon: vec!["TGT", "TGC"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Glutamic Acid" => Self {
                name: "Glutamic Acid".to_string(),
                short_name: "Glu".to_string(),
                abbreviation: "E".to_string(),
                side_chain: "Acidic".to_string(),
                molecular_weight: 147.13,
                codon: vec!["GAA", "GAG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Glutamine" => Self {
                name: "Glutamine".to_string(),
                short_name: "Gln".to_string(),
                abbreviation: "Q".to_string(),
                side_chain: "Polar".to_string(),
                molecular_weight: 146.15,
                codon: vec!["CAA", "CAG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Glycine" => Self {
                name: "Glycine".to_string(),
                short_name: "Gly".to_string(),
                abbreviation: "G".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 75.07,
                codon: vec!["GGT", "GGC", "GGA", "GGG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Histidine" => Self {
                name: "Histidine".to_string(),
                short_name: "His".to_string(),
                abbreviation: "H".to_string(),
                side_chain: "Positive".to_string(),
                molecular_weight: 155.16,
                codon: vec!["CAT", "CAC"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Isoleucine" => Self {
                name: "Isoleucine".to_string(),
                short_name: "Ile".to_string(),
                abbreviation: "I".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 131.18,
                codon: vec!["ATT", "ATC", "ATA"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Leucine" => Self {
                name: "Leucine".to_string(),
                short_name: "Leu".to_string(),
                abbreviation: "L".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 131.18,
                codon: vec!["TTA", "CTT", "CTC", "CTG", "CTA"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Lysine" => Self {
                name: "Lysine".to_string(),
                short_name: "Lys".to_string(),
                abbreviation: "K".to_string(),
                side_chain: "Basic".to_string(),
                molecular_weight: 146.19,
                codon: vec!["AAA", "AAG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Methionine" => Self {
                name: "Methionine".to_string(),
                short_name: "Met".to_string(),
                abbreviation: "M".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 149.21,
                codon: vec!["ATG"].into_iter().map(ToString::to_string).collect(),
            },
            "Phenylalanine" => Self {
                name: "Phenylalanine".to_string(),
                short_name: "Phe".to_string(),
                abbreviation: "F".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 165.19,
                codon: vec!["TTT", "TTC"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Proline" => Self {
                name: "Proline".to_string(),
                short_name: "Pro".to_string(),
                abbreviation: "P".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 115.13,
                codon: vec!["CCT", "CCC", "CCA", "CCG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Serine" => Self {
                name: "Serine".to_string(),
                short_name: "Ser".to_string(),
                abbreviation: "S".to_string(),
                side_chain: "Polar".to_string(),
                molecular_weight: 105.09,
                codon: vec!["TCT", "TCC", "TCA", "TCG", "AGT", "AGC"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Threonine" => Self {
                name: "Threonine".to_string(),
                short_name: "Thr".to_string(),
                abbreviation: "T".to_string(),
                side_chain: "Polar".to_string(),
                molecular_weight: 119.12,
                codon: vec!["ACT", "ACC", "ACA", "ACG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Tryptophan" => Self {
                name: "Tryptophan".to_string(),
                short_name: "Trp".to_string(),
                abbreviation: "W".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 204.23,
                codon: vec!["TGG"].into_iter().map(ToString::to_string).collect(),
            },
            "Tyrosine" => Self {
                name: "Tyrosine".to_string(),
                short_name: "Tyr".to_string(),
                abbreviation: "Y".to_string(),
                side_chain: "Polar".to_string(),
                molecular_weight: 181.19,
                codon: vec!["TAT", "TAC"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            "Valine" => Self {
                name: "Valine".to_string(),
                short_name: "Val".to_string(),
                abbreviation: "V".to_string(),
                side_chain: "Nonpolar".to_string(),
                molecular_weight: 117.15,
                codon: vec!["GTT", "GTC", "GTA", "GTG"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            _ => Self {
                name: "Unknown".to_string(),
                short_name: "Unknown".to_string(),
                abbreviation: "X".to_string(),
                side_chain: "Unknown".to_string(),
                molecular_weight: 0.0,
                codon: vec!["---"].into_iter().map(ToString::to_string).collect(),
            },
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_short_name(&self) -> String {
        self.short_name.clone()
    }

    pub fn get_abbreviation(&self) -> String {
        self.abbreviation.clone()
    }

    pub fn get_side_chain(&self) -> String {
        self.side_chain.clone()
    }

    pub const fn get_molecular_weight(&self) -> f64 {
        self.molecular_weight
    }

    pub fn get_codon(&self) -> Vec<String> {
        self.codon.clone()
    }

    pub fn get_codon_string(&self) -> String {
        self.codon.join(", ")
    }

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
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_name(), "Alanine");
    }
    #[test]
    fn test_get_short_name() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_short_name(), "Ala");
    }
    #[test]
    fn test_get_abbreviation() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_abbreviation(), "A");
    }
    #[test]
    fn test_get_side_chain() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_side_chain(), "Nonpolar");
    }
    #[test]
    fn test_get_molecular_weight() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_molecular_weight(), 89.09);
    }
    #[test]
    fn test_get_codon() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_codon(), vec!["GCT", "GCC", "GCA", "GCG"]);
    }
    #[test]
    fn test_get_codon_string() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_codon_string(), "GCT, GCC, GCA, GCG");
    }
    #[test]
    fn test_get_codon_count() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(amino_acid.get_codon_count(), 4);
    }

    #[test]
    fn test_fmt() {
        let amino_acid = AminoAcid::new("Alanine");
        assert_eq!(
            format!("{}", amino_acid),
            "Name: Alanine\tShort Name: Ala\tAbbreviation: A\tSide Chain: Nonpolar\tMolecular Weight: 89.09\tCodon: GCT, GCC, GCA, GCG"
        );
    }
}
