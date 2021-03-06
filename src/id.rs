// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

/// The public identity of a node.  It provides functionality to allow it to be used as an
/// asymmetric signing public key.
pub trait PublicId:
    'static + Send + Debug + Clone + Eq + Ord + Hash + Serialize + DeserializeOwned
{
    /// The signature type associated with the chosen asymmetric key scheme.
    type Signature: 'static + Send + Debug + Clone + Eq + Ord + Hash + Serialize + DeserializeOwned;
    /// Verify `signature` against `data` using this `PublicId`.  Returns `true` if valid.
    fn verify_detached(&self, signature: &Self::Signature, data: &[u8]) -> bool;
}

/// The secret identity of a node.  It provides functionality to allow it to be used as an
/// asymmetric signing secret key and to also yield the associated public identity.
pub trait SecretId {
    /// The associated public identity type.
    type PublicId: PublicId;

    /// Returns the associated public identity.
    fn pub_id(&self) -> &Self::PublicId;

    /// Creates a detached `Signature` of `data`.
    fn sign_detached(&self, data: &[u8]) -> <Self::PublicId as PublicId>::Signature;

    /// Creates a `Proof` of `data`.
    fn create_proof(&self, data: &[u8]) -> Proof<Self::PublicId> {
        Proof {
            pub_id: self.pub_id().clone(),
            signature: self.sign_detached(data),
        }
    }
}

/// A basic helper to carry a given [`Signature`](trait.PublicId.html#associatedtype.Signature)
/// along with the signer's [`PublicId`](trait.PublicId.html).
#[serde(bound = "")]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Proof<P: PublicId> {
    pub_id: P,
    signature: P::Signature,
}
