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

        pub enum CosignatoryModificationBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct CosignatoryModificationBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for CosignatoryModificationBuffer<'a> {
            type Inner = CosignatoryModificationBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> CosignatoryModificationBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                CosignatoryModificationBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args CosignatoryModificationBufferArgs<'args>) -> fb::WIPOffset<CosignatoryModificationBuffer<'bldr>> {
                let mut builder = CosignatoryModificationBufferBuilder::new(_fbb);
                if let Some(x) = args.cosignatoryPublicKey { builder.add_cosignatoryPublicKey(x); }
                builder.add_type_(args.type_);
                builder.finish()
            }

            pub const VT_TYPE_: fb::VOffsetT = 4;
            pub const VT_COSIGNATORYPUBLICKEY: fb::VOffsetT = 6;

            #[inline]
            pub fn type_(&self) -> u8 {
                self._tab.get::<u8>(CosignatoryModificationBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn cosignatoryPublicKey(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(CosignatoryModificationBuffer::VT_COSIGNATORYPUBLICKEY, None).map(|v| v.safe_slice())
            }
        }

        pub struct CosignatoryModificationBufferArgs<'a> {
            pub type_: u8,
            pub cosignatoryPublicKey: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        }

        impl<'a> Default for CosignatoryModificationBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                CosignatoryModificationBufferArgs {
                    type_: 0,
                    cosignatoryPublicKey: None,
                }
            }
        }

        pub struct CosignatoryModificationBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> CosignatoryModificationBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_type_(&mut self, type_: u8) {
                self.fbb_.push_slot::<u8>(CosignatoryModificationBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_cosignatoryPublicKey(&mut self, cosignatoryPublicKey: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(CosignatoryModificationBuffer::VT_COSIGNATORYPUBLICKEY, cosignatoryPublicKey);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> CosignatoryModificationBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                CosignatoryModificationBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<CosignatoryModificationBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        pub enum ModifyContractTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct ModifyContractTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for ModifyContractTransactionBuffer<'a> {
            type Inner = ModifyContractTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> ModifyContractTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                ModifyContractTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args ModifyContractTransactionBufferArgs<'args>) -> fb::WIPOffset<ModifyContractTransactionBuffer<'bldr>> {
                let mut builder = ModifyContractTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.verifiers { builder.add_verifiers(x); }
                if let Some(x) = args.executors { builder.add_executors(x); }
                if let Some(x) = args.customers { builder.add_customers(x); }
                if let Some(x) = args.hash { builder.add_hash(x); }
                if let Some(x) = args.durationDelta { builder.add_durationDelta(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.maxFee { builder.add_maxFee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_numVerifiers(args.numVerifiers);
                builder.add_numExecutors(args.numExecutors);
                builder.add_numCustomers(args.numCustomers);
                builder.finish()
            }

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_DURATIONDELTA: fb::VOffsetT = 18;
            pub const VT_HASH: fb::VOffsetT = 20;
            pub const VT_NUMCUSTOMERS: fb::VOffsetT = 22;
            pub const VT_NUMEXECUTORS: fb::VOffsetT = 24;
            pub const VT_NUMVERIFIERS: fb::VOffsetT = 26;
            pub const VT_CUSTOMERS: fb::VOffsetT = 28;
            pub const VT_EXECUTORS: fb::VOffsetT = 30;
            pub const VT_VERIFIERS: fb::VOffsetT = 32;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(ModifyContractTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(ModifyContractTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(ModifyContractTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(ModifyContractTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(ModifyContractTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn maxFee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(ModifyContractTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(ModifyContractTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn durationDelta(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(ModifyContractTransactionBuffer::VT_DURATIONDELTA, None)
            }
            #[inline]
            pub fn hash(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(ModifyContractTransactionBuffer::VT_HASH, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn numCustomers(&self) -> u8 {
                self._tab.get::<u8>(ModifyContractTransactionBuffer::VT_NUMCUSTOMERS, Some(0)).unwrap()
            }
            #[inline]
            pub fn numExecutors(&self) -> u8 {
                self._tab.get::<u8>(ModifyContractTransactionBuffer::VT_NUMEXECUTORS, Some(0)).unwrap()
            }
            #[inline]
            pub fn numVerifiers(&self) -> u8 {
                self._tab.get::<u8>(ModifyContractTransactionBuffer::VT_NUMVERIFIERS, Some(0)).unwrap()
            }
            #[inline]
            pub fn customers(&self) -> Option<fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>>>(ModifyContractTransactionBuffer::VT_CUSTOMERS, None)
            }
            #[inline]
            pub fn executors(&self) -> Option<fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>>>(ModifyContractTransactionBuffer::VT_EXECUTORS, None)
            }
            #[inline]
            pub fn verifiers(&self) -> Option<fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>>>(ModifyContractTransactionBuffer::VT_VERIFIERS, None)
            }
        }

        pub struct ModifyContractTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: i32,
            pub type_: u16,
            pub maxFee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub durationDelta: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub hash: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub numCustomers: u8,
            pub numExecutors: u8,
            pub numVerifiers: u8,
            pub customers: Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>>>,
            pub executors: Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>>>,
            pub verifiers: Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>>>,
        }

        impl<'a> Default for ModifyContractTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                ModifyContractTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    maxFee: None,
                    deadline: None,
                    durationDelta: None,
                    hash: None,
                    numCustomers: 0,
                    numExecutors: 0,
                    numVerifiers: 0,
                    customers: None,
                    executors: None,
                    verifiers: None,
                }
            }
        }

        pub struct ModifyContractTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> ModifyContractTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(ModifyContractTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: i32) {
                self.fbb_.push_slot::<u32>(ModifyContractTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(ModifyContractTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_maxFee(&mut self, maxFee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_MAXFEE, maxFee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_durationDelta(&mut self, durationDelta: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_DURATIONDELTA, durationDelta);
            }
            #[inline]
            pub fn add_hash(&mut self, hash: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_HASH, hash);
            }
            #[inline]
            pub fn add_numCustomers(&mut self, numCustomers: u8) {
                self.fbb_.push_slot::<u8>(ModifyContractTransactionBuffer::VT_NUMCUSTOMERS, numCustomers, 0);
            }
            #[inline]
            pub fn add_numExecutors(&mut self, numExecutors: u8) {
                self.fbb_.push_slot::<u8>(ModifyContractTransactionBuffer::VT_NUMEXECUTORS, numExecutors, 0);
            }
            #[inline]
            pub fn add_numVerifiers(&mut self, numVerifiers: u8) {
                self.fbb_.push_slot::<u8>(ModifyContractTransactionBuffer::VT_NUMVERIFIERS, numVerifiers, 0);
            }
            #[inline]
            pub fn add_customers(&mut self, customers: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<CosignatoryModificationBuffer<'b>>>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_CUSTOMERS, customers);
            }
            #[inline]
            pub fn add_executors(&mut self, executors: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<CosignatoryModificationBuffer<'b>>>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_EXECUTORS, executors);
            }
            #[inline]
            pub fn add_verifiers(&mut self, verifiers: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<CosignatoryModificationBuffer<'b>>>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(ModifyContractTransactionBuffer::VT_VERIFIERS, verifiers);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> ModifyContractTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                ModifyContractTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<ModifyContractTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_modify_contract_transaction_buffer<'a>(buf: &'a [u8]) -> ModifyContractTransactionBuffer<'a> {
            fb::get_root::<ModifyContractTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_modify_contract_transaction_buffer<'a>(buf: &'a [u8]) -> ModifyContractTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<ModifyContractTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_modify_contract_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<ModifyContractTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_modify_contract_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut fb::FlatBufferBuilder<'a>, root: fb::WIPOffset<ModifyContractTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

