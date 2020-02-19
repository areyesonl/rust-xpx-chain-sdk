// automatically generated by the FlatBuffers compiler, do not modify



extern crate flatbuffers;

use std::cmp::Ordering;
use std::mem;

use self::flatbuffers::EndianScalar;


#[allow(unused_imports, dead_code)]
pub mod catapult {
    use std::cmp::Ordering;
    use std::mem;

    use self::flatbuffers::EndianScalar;

    extern crate flatbuffers;

    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        use std::cmp::Ordering;
        use std::mem;

        use self::flatbuffers::EndianScalar;

        extern crate flatbuffers;

        pub enum SecretProofTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct SecretProofTransactionBuffer<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for SecretProofTransactionBuffer<'a> {
            type Inner = SecretProofTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> SecretProofTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                SecretProofTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args SecretProofTransactionBufferArgs<'args>) -> flatbuffers::WIPOffset<SecretProofTransactionBuffer<'bldr>> {
                let mut builder = SecretProofTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.proof { builder.add_proof(x); }
                if let Some(x) = args.recipient { builder.add_recipient(x); }
                if let Some(x) = args.secret { builder.add_secret(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.maxFee { builder.add_maxFee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_proofSize(args.proofSize);
                builder.add_type_(args.type_);
                builder.add_hashAlgorithm(args.hashAlgorithm);
                builder.finish()
            }

            pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
            pub const VT_SIGNATURE: flatbuffers::VOffsetT = 6;
            pub const VT_SIGNER: flatbuffers::VOffsetT = 8;
            pub const VT_VERSION: flatbuffers::VOffsetT = 10;
            pub const VT_TYPE_: flatbuffers::VOffsetT = 12;
            pub const VT_MAXFEE: flatbuffers::VOffsetT = 14;
            pub const VT_DEADLINE: flatbuffers::VOffsetT = 16;
            pub const VT_HASHALGORITHM: flatbuffers::VOffsetT = 18;
            pub const VT_SECRET: flatbuffers::VOffsetT = 20;
            pub const VT_RECIPIENT: flatbuffers::VOffsetT = 22;
            pub const VT_PROOFSIZE: flatbuffers::VOffsetT = 24;
            pub const VT_PROOF: flatbuffers::VOffsetT = 26;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(SecretProofTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(SecretProofTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(SecretProofTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(SecretProofTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(SecretProofTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn maxFee(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(SecretProofTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(SecretProofTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn hashAlgorithm(&self) -> u8 {
                self._tab.get::<u8>(SecretProofTransactionBuffer::VT_HASHALGORITHM, Some(0)).unwrap()
            }
            #[inline]
            pub fn secret(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(SecretProofTransactionBuffer::VT_SECRET, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn recipient(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(SecretProofTransactionBuffer::VT_RECIPIENT, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn proofSize(&self) -> u16 {
                self._tab.get::<u16>(SecretProofTransactionBuffer::VT_PROOFSIZE, Some(0)).unwrap()
            }
            #[inline]
            pub fn proof(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(SecretProofTransactionBuffer::VT_PROOF, None).map(|v| v.safe_slice())
            }
        }

        pub struct SecretProofTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub signer: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub maxFee: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub deadline: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub hashAlgorithm: u8,
            pub secret: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub recipient: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub proofSize: u16,
            pub proof: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        }

        impl<'a> Default for SecretProofTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                SecretProofTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    maxFee: None,
                    deadline: None,
                    hashAlgorithm: 0,
                    secret: None,
                    recipient: None,
                    proofSize: 0,
                    proof: None,
                }
            }
        }

        pub struct SecretProofTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> SecretProofTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(SecretProofTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SecretProofTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SecretProofTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(SecretProofTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(SecretProofTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_maxFee(&mut self, maxFee: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SecretProofTransactionBuffer::VT_MAXFEE, maxFee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SecretProofTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_hashAlgorithm(&mut self, hashAlgorithm: u8) {
                self.fbb_.push_slot::<u8>(SecretProofTransactionBuffer::VT_HASHALGORITHM, hashAlgorithm, 0);
            }
            #[inline]
            pub fn add_secret(&mut self, secret: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SecretProofTransactionBuffer::VT_SECRET, secret);
            }
            #[inline]
            pub fn add_recipient(&mut self, recipient: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SecretProofTransactionBuffer::VT_RECIPIENT, recipient);
            }
            #[inline]
            pub fn add_proofSize(&mut self, proofSize: u16) {
                self.fbb_.push_slot::<u16>(SecretProofTransactionBuffer::VT_PROOFSIZE, proofSize, 0);
            }
            #[inline]
            pub fn add_proof(&mut self, proof: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SecretProofTransactionBuffer::VT_PROOF, proof);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SecretProofTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SecretProofTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<SecretProofTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_secret_proof_transaction_buffer<'a>(buf: &'a [u8]) -> SecretProofTransactionBuffer<'a> {
            flatbuffers::get_root::<SecretProofTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_secret_proof_transaction_buffer<'a>(buf: &'a [u8]) -> SecretProofTransactionBuffer<'a> {
            flatbuffers::get_size_prefixed_root::<SecretProofTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_secret_proof_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<SecretProofTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_secret_proof_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<SecretProofTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

