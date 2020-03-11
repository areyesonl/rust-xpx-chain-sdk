extern crate flatbuffers;

#[allow(unused_imports, dead_code)]
pub mod lock_funds {

    extern crate flatbuffers;

    #[allow(unused_imports, dead_code)]
    pub mod buffers {

        extern crate flatbuffers;

        pub enum LockFundsTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct LockFundsTransactionBuffer<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for LockFundsTransactionBuffer<'a> {
            type Inner = LockFundsTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf, loc },
                }
            }
        }

        impl<'a> LockFundsTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                LockFundsTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args LockFundsTransactionBufferArgs<'args>) -> flatbuffers::WIPOffset<LockFundsTransactionBuffer<'bldr>> {
                let mut builder = LockFundsTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.hash { builder.add_hash(x); }
                if let Some(x) = args.duration { builder.add_duration(x); }
                if let Some(x) = args.mosaic_amount { builder.add_mosaic_amount(x); }
                if let Some(x) = args.mosaic_id { builder.add_mosaic_id(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.max_fee { builder.add_max_fee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.finish()
            }

            pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
            pub const VT_SIGNATURE: flatbuffers::VOffsetT = 6;
            pub const VT_SIGNER: flatbuffers::VOffsetT = 8;
            pub const VT_VERSION: flatbuffers::VOffsetT = 10;
            pub const VT_TYPE_: flatbuffers::VOffsetT = 12;
            pub const VT_MAXFEE: flatbuffers::VOffsetT = 14;
            pub const VT_DEADLINE: flatbuffers::VOffsetT = 16;
            pub const VT_MOSAICID: flatbuffers::VOffsetT = 18;
            pub const VT_MOSAICAMOUNT: flatbuffers::VOffsetT = 20;
            pub const VT_DURATION: flatbuffers::VOffsetT = 22;
            pub const VT_HASH: flatbuffers::VOffsetT = 24;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(LockFundsTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(LockFundsTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(LockFundsTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(LockFundsTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(LockFundsTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn max_fee(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(LockFundsTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(LockFundsTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn mosaic_id(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(LockFundsTransactionBuffer::VT_MOSAICID, None)
            }
            #[inline]
            pub fn mosaic_amount(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(LockFundsTransactionBuffer::VT_MOSAICAMOUNT, None)
            }
            #[inline]
            pub fn duration(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(LockFundsTransactionBuffer::VT_DURATION, None)
            }
            #[inline]
            pub fn hash(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(LockFundsTransactionBuffer::VT_HASH, None).map(|v| v.safe_slice())
            }
        }

        pub struct LockFundsTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub signer: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub deadline: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub mosaic_id: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub mosaic_amount: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub duration: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub hash: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        }

        impl<'a> Default for LockFundsTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                LockFundsTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    max_fee: None,
                    deadline: None,
                    mosaic_id: None,
                    mosaic_amount: None,
                    duration: None,
                    hash: None,
                }
            }
        }

        pub struct LockFundsTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> LockFundsTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(LockFundsTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(LockFundsTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(LockFundsTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_max_fee(&mut self, max_fee: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_MAXFEE, max_fee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_mosaic_id(&mut self, mosaic_id: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_MOSAICID, mosaic_id);
            }
            #[inline]
            pub fn add_mosaic_amount(&mut self, mosaic_amount: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_MOSAICAMOUNT, mosaic_amount);
            }
            #[inline]
            pub fn add_duration(&mut self, duration: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_DURATION, duration);
            }
            #[inline]
            pub fn add_hash(&mut self, hash: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(LockFundsTransactionBuffer::VT_HASH, hash);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LockFundsTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                LockFundsTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<LockFundsTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_lock_funds_transaction_buffer<'a>(buf: &'a [u8]) -> LockFundsTransactionBuffer<'a> {
            flatbuffers::get_root::<LockFundsTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_lock_funds_transaction_buffer<'a>(buf: &'a [u8]) -> LockFundsTransactionBuffer<'a> {
            flatbuffers::get_size_prefixed_root::<LockFundsTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_lock_funds_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<LockFundsTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_lock_funds_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<LockFundsTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

