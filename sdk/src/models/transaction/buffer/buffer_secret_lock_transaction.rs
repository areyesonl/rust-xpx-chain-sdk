// automatically generated by the FlatBuffers compiler, do not modify



extern crate flatbuffers;

use std::cmp::Ordering;
use std::mem;

use self::fb::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod catapult {
    use std::cmp::Ordering;
    use std::mem;

    use self::fb::EndianScalar;

    extern crate flatbuffers;

    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        use std::cmp::Ordering;
        use std::mem;

        use self::fb::EndianScalar;

        extern crate flatbuffers;

        pub enum SecretLockTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct SecretLockTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for SecretLockTransactionBuffer<'a> {
            type Inner = SecretLockTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> SecretLockTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                SecretLockTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args SecretLockTransactionBufferArgs<'args>) -> fb::WIPOffset<SecretLockTransactionBuffer<'bldr>> {
                let mut builder = SecretLockTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.recipient { builder.add_recipient(x); }
                if let Some(x) = args.secret { builder.add_secret(x); }
                if let Some(x) = args.duration { builder.add_duration(x); }
                if let Some(x) = args.mosaicAmount { builder.add_mosaicAmount(x); }
                if let Some(x) = args.mosaicId { builder.add_mosaicId(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.maxFee { builder.add_maxFee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_hashAlgorithm(args.hashAlgorithm);
                builder.finish()
            }

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_MOSAICID: fb::VOffsetT = 18;
            pub const VT_MOSAICAMOUNT: fb::VOffsetT = 20;
            pub const VT_DURATION: fb::VOffsetT = 22;
            pub const VT_HASHALGORITHM: fb::VOffsetT = 24;
            pub const VT_SECRET: fb::VOffsetT = 26;
            pub const VT_RECIPIENT: fb::VOffsetT = 28;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(SecretLockTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(SecretLockTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(SecretLockTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(SecretLockTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(SecretLockTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn maxFee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(SecretLockTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(SecretLockTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn mosaicId(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(SecretLockTransactionBuffer::VT_MOSAICID, None)
            }
            #[inline]
            pub fn mosaicAmount(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(SecretLockTransactionBuffer::VT_MOSAICAMOUNT, None)
            }
            #[inline]
            pub fn duration(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(SecretLockTransactionBuffer::VT_DURATION, None)
            }
            #[inline]
            pub fn hashAlgorithm(&self) -> u8 {
                self._tab.get::<u8>(SecretLockTransactionBuffer::VT_HASHALGORITHM, Some(0)).unwrap()
            }
            #[inline]
            pub fn secret(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(SecretLockTransactionBuffer::VT_SECRET, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn recipient(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(SecretLockTransactionBuffer::VT_RECIPIENT, None).map(|v| v.safe_slice())
            }
        }

        pub struct SecretLockTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: i32,
            pub type_: u16,
            pub maxFee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub mosaicId: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub mosaicAmount: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub duration: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub hashAlgorithm: u8,
            pub secret: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub recipient: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        }

        impl<'a> Default for SecretLockTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                SecretLockTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    maxFee: None,
                    deadline: None,
                    mosaicId: None,
                    mosaicAmount: None,
                    duration: None,
                    hashAlgorithm: 0,
                    secret: None,
                    recipient: None,
                }
            }
        }

        pub struct SecretLockTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> SecretLockTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(SecretLockTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: i32) {
                self.fbb_.push_slot::<u32>(SecretLockTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(SecretLockTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_maxFee(&mut self, maxFee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_MAXFEE, maxFee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_mosaicId(&mut self, mosaicId: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_MOSAICID, mosaicId);
            }
            #[inline]
            pub fn add_mosaicAmount(&mut self, mosaicAmount: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_MOSAICAMOUNT, mosaicAmount);
            }
            #[inline]
            pub fn add_duration(&mut self, duration: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_DURATION, duration);
            }
            #[inline]
            pub fn add_hashAlgorithm(&mut self, hashAlgorithm: u8) {
                self.fbb_.push_slot::<u8>(SecretLockTransactionBuffer::VT_HASHALGORITHM, hashAlgorithm, 0);
            }
            #[inline]
            pub fn add_secret(&mut self, secret: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_SECRET, secret);
            }
            #[inline]
            pub fn add_recipient(&mut self, recipient: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(SecretLockTransactionBuffer::VT_RECIPIENT, recipient);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> SecretLockTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SecretLockTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<SecretLockTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_secret_lock_transaction_buffer<'a>(buf: &'a [u8]) -> SecretLockTransactionBuffer<'a> {
            fb::get_root::<SecretLockTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_secret_lock_transaction_buffer<'a>(buf: &'a [u8]) -> SecretLockTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<SecretLockTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_secret_lock_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<SecretLockTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_secret_lock_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut fb::FlatBufferBuilder<'a>, root: fb::WIPOffset<SecretLockTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

