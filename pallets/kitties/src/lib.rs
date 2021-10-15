#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{sp_runtime::traits::{Hash, Zero},
                        dispatch::{DispatchResultWithPostInfo, DispatchResult},
                        traits::{Currency, ExistenceRequirement, Randomness},
                        pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;


    // TODO Part II: Struct for holding Kitty information.

    // TODO Part II: Enum and implementation to handle Gender type in Kitty struct.

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the Kitties pallet.
        type Currency: Currency<Self::AccountId>;

        // TODO Part II: Specify the custom types for our runtime.

    }

    // Our pallet's genesis configuration.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub kitties: Vec<(T::AccountId, [u8; 16])>,
    }

    // Required to implement default for GenesisConfig.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> GenesisConfig<T> {
            GenesisConfig { kitties: vec![] }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            // When building a kitty from genesis config, we require the dna and gender to be supplied.
            // for (acct, dna, gender) in &self.kitties {
            //     let _ = <Pallet<T>>::mint(acct, Some(dna.clone()), Some(gender.clone()));
            // }
        }
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // TODO Part III
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO Part III
    }

    // ACTION: Storage item to keep a count of all existing Kitties.
    #[pallet::storage]
    #[pallet::getter(fn kitty_cnt)]
    /// Keeps track of the number of Kitties in existence.
    pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

    // TODO Part II: Remaining storage items.

    // TODO Part III: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // TODO Part III: create_kitty

        // TODO Part III: set_price

        // TODO Part III: transfer

        // TODO Part III: buy_kitty

        // TODO Part III: breed_kitty
    }

    // TODO Parts II: helper function for Kitty struct

    impl<T: Config> Pallet<T> {
        // TODO Part III: helper functions for dispatchable functions

        // TODO: increment_nonce, random_hash, mint, transfer_from

    }
}