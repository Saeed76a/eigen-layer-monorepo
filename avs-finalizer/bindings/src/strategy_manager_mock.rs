pub use strategy_manager_mock::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod strategy_manager_mock {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addShares"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addStrategiesToDepositWhitelist"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addStrategiesToDepositWhitelist",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beaconChainETHStrategy"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("beaconChainETHStrategy",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateWithdrawalRoot"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("calculateWithdrawalRoot",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("queuedWithdrawal"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStrategyManager.DeprecatedStruct_QueuedWithdrawal",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cumulativeWithdrawalsQueued"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cumulativeWithdrawalsQueued",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegation"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IDelegationManager",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositBeaconChainETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositBeaconChainETH",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositIntoStrategy"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositIntoStrategy",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositIntoStrategyWithSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositIntoStrategyWithSignature",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("shares"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IEigenPodManager",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDeposits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDeposits"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("migrateQueuedWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("migrateQueuedWithdrawal",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("queuedWithdrawal"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStrategyManager.DeprecatedStruct_QueuedWithdrawal",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pause"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordBeaconChainETHBalanceUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("recordBeaconChainETHBalanceUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("overcommittedPodOwner",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "beaconChainETHStrategyIndex",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sharesDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeShares"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeStrategiesFromDepositWhitelist"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "removeStrategiesFromDepositWhitelist",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAddresses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAddresses"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_delegation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IDelegationManager",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_eigenPodManager"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IEigenPodManager",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_slasher"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDeposits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setDeposits"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_strategiesToReturn",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_sharesToReturn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setStakerStrategyListLengthReturnValue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "setStakerStrategyListLengthReturnValue",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("valueToSet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sharesToReturn"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sharesToReturn"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("slasher"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("slasher"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerStrategyListLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakerStrategyListLength",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerStrategyListLengthReturnValue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "stakerStrategyListLengthReturnValue",
                        ),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerStrategyShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakerStrategyShares",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("shares"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerStrats"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakerStrats"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("staker"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategiesToReturn"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategiesToReturn"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategyWhitelister"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategyWhitelister",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unpause"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawSharesAsTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawSharesAsTokens",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Paused"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyAddedToDepositWhitelist"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StrategyAddedToDepositWhitelist",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyRemovedFromDepositWhitelist"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned(
                            "StrategyRemovedFromDepositWhitelist",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyWhitelisterChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StrategyWhitelisterChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STRATEGYMANAGERMOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x16,\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02'W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x010W\x80c\xB4;QK\x11a\0\xB8W\x80c\xCDGL\x8A\x11a\0|W\x80c\xCDGL\x8A\x14a\x04\xFAW\x80c\xDF\\\xF7#\x14a\x05\rW\x80c\xE7\xA0P\xAA\x14a\x05 W\x80c\xF2\xFD\xE3\x8B\x14a\x057W\x80c\xFA\xBC\x1C\xBC\x14a\x05JW`\0\x80\xFD[\x80c\xB4;QK\x14a\x04\x92W\x80c\xB5\xD8\xB5\xB8\x14a\x03yW\x80c\xC6\x08\xC7\xF3\x14a\x04\xA6W\x80c\xC7;B\xBE\x14a\x04\xBAW\x80c\xCD)?o\x14a\x04\xCDW`\0\x80\xFD[\x80c\x9A\x95\x19\xE0\x11a\0\xFFW\x80c\x9A\x95\x19\xE0\x14a\x040W\x80c\x9F\0\xFA$\x14a\x04CW\x80c\xA1x\x84\x84\x14a\x04QW\x80c\xA1\xCAx\x0B\x14a\x04qW\x80c\xB14Bq\x14a\x04\x7FW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x03\xE4W\x80c\x91\x04\xC3\x19\x14a\x03\xF5W\x80c\x94\xF6I\xDD\x14a\x03\xFCW\x80c\x96\x7F\xC0\xD2\x14a\x04\x1DW`\0\x80\xFD[\x80cZ\xC8j\xB7\x11a\x01\xB3W\x80cz~\r\x92\x11a\x01\x82W\x80cz~\r\x92\x14a\x03\x93W\x80cz\xB8\x01\xD1\x14a\x03\xA9W\x80c\x88o\x11\x95\x14a\x03\xBCW\x80c\x8B\x8A\xAC<\x14a\x03\xCFW\x80c\x8C\x80\xD4\xE5\x14a\x03#W`\0\x80\xFD[\x80cZ\xC8j\xB7\x14a\x03>W\x80c\\\x97Z\xBB\x14a\x03qW\x80c]\xE0\x8F\xF2\x14a\x03yW\x80cqP\x18\xA6\x14a\x03\x8BW`\0\x80\xFD[\x80c2\xE8\x9A\xCE\x11a\x01\xFAW\x80c2\xE8\x9A\xCE\x14a\x02\x91W\x80c6;\xF9d\x14a\x02\xABW\x80cFe\xBC\xDA\x14a\x02\xF8W\x80cP\xFFr%\x14a\x03#W\x80cY\\jg\x14a\x036W`\0\x80\xFD[\x80c\x01\xF8 \xB2\x14a\x02,W\x80c\r9\x08\xF4\x14a\x02HW\x80c\x10\xD6z/\x14a\x02iW\x80c\x13d9\xDD\x14a\x02~W[`\0\x80\xFD[a\x025`\xD0T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\\a\x02V6`\x04a\x0E\x04V[P``\x90V[`@Qa\x02?\x91\x90a\x0ElV[a\x02|a\x02w6`\x04a\x0E\x04V[a\x05]V[\0[a\x02|a\x02\x8C6`\x04a\x0E\x7FV[a\x06\x19V[a\x025a\x02\x9F6`\x04a\x0F\x08V[`\0\x96\x95PPPPPPV[a\x02|a\x02\xB96`\x04a\x0F\xE7V[`\xC9\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\xCB\x80T\x92\x85\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\xCA\x80T\x92\x90\x93\x16\x91\x16\x17\x90UV[`\xCATa\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02?V[a\x02|a\x0316`\x04a\x102V[PPPV[a\x02|a\x07XV[a\x03aa\x03L6`\x04a\x10sV[`\x98T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02?V[`\x98Ta\x025V[a\x02|a\x03\x876`\x04a\x10\xE2V[PPV[a\x02|a\x08\x1FV[a\x025a\x03\xA16`\x04a\x11$V[`\0\x92\x91PPV[a\x03\x0Ba\x03\xB76`\x04a\x0E\x7FV[a\x083V[`\x97Ta\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x025a\x03\xDD6`\x04a\x0E\x04V[P`\xD0T\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x0BV[`\0a\x03\x0BV[a\x04\x0Fa\x04\n6`\x04a\x0E\x04V[a\x08]V[`@Qa\x02?\x92\x91\x90a\x11]V[`\xCCTa\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02|a\x04>6`\x04a\x0E\x7FV[`\xD0UV[a\x02|a\x03\x876`\x04a\x11\xB4V[a\x025a\x04_6`\x04a\x0E\x04V[`\xCF` R`\0\x90\x81R`@\x90 T\x81V[a\x02|a\x0316`\x04a\x11\xE0V[`\xCBTa\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x025a\x04\xA06`\x04a\x13\x90V[P`\0\x90V[a\x02|a\x04\xB46`\x04a\x14fV[PPPPV[a\x02|a\x04\xC86`\x04a\x14\xB9V[a\t\x1BV[a\x04\xE3a\x04\xDB6`\x04a\x13\x90V[`\0\x80\x91P\x91V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02?V[a\x025a\x05\x086`\x04a\x0E\x7FV[a\t\x96V[`\xC9Ta\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x025a\x05.6`\x04a\x102V[`\0\x93\x92PPPV[a\x02|a\x05E6`\x04a\x0E\x04V[a\t\xB7V[a\x02|a\x05X6`\x04a\x0E\x7FV[a\n-V[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD4\x91\x90a\x15%V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15BV[`@Q\x80\x91\x03\x90\xFD[a\x06\x16\x81a\x0B\x89V[PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x85\x91\x90a\x15\x8CV[a\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15\xAEV[`\x98T\x81\x81\x16\x14a\x07\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x04V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xC4\x91\x90a\x15\x8CV[a\x07\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15\xAEV[`\0\x19`\x98\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x08'a\x0C\x80V[a\x081`\0a\x0C\xDAV[V[`\xCD\x81\x81T\x81\x10a\x08CW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[``\x80`\xCD`\xCE\x81\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xB9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x9BW[PPPPP\x91P\x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x0BW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xF7W[PPPPP\x90P\x91P\x91P\x91P\x91V[\x82\x81\x14a\tvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FStrategyManagerMock: length mism`D\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`d\x82\x01R`\x84\x01a\x06\x04V[a\t\x82`\xCD\x85\x85a\r,V[Pa\t\x8F`\xCE\x83\x83a\r\x8FV[PPPPPV[`\xCE\x81\x81T\x81\x10a\t\xA6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[a\t\xBFa\x0C\x80V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\x04V[a\x06\x16\x81a\x0C\xDAV[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA4\x91\x90a\x15%V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15BV[`\x98T\x19\x81\x19`\x98T\x19\x16\x14a\x0BRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x04V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07MV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\x04V[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x081W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\x04V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\r\x7FW\x91` \x02\x82\x01[\x82\x81\x11\x15a\r\x7FW\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x845\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\rLV[Pa\r\x8B\x92\x91Pa\r\xCAV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\r\x7FW\x91` \x02\x82\x01[\x82\x81\x11\x15a\r\x7FW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a\r\xAFV[[\x80\x82\x11\x15a\r\x8BW`\0\x81U`\x01\x01a\r\xCBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x16W`\0\x80\xFD[\x805a\r\xFF\x81a\r\xDFV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\x16W`\0\x80\xFD[\x815a\x0E!\x81a\r\xDFV[\x93\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x0EaW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0E<V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x0E!` \x83\x01\x84a\x0E(V[`\0` \x82\x84\x03\x12\x15a\x0E\x91W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xD1Wa\x0E\xD1a\x0E\x98V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\0Wa\x0F\0a\x0E\x98V[`@R\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0F!W`\0\x80\xFD[\x865a\x0F,\x81a\r\xDFV[\x95P` \x87\x81\x015a\x0F=\x81a\r\xDFV[\x95P`@\x88\x015\x94P``\x88\x015a\x0FT\x81a\r\xDFV[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FxW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x0F\x8CW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\x9EWa\x0F\x9Ea\x0E\x98V[a\x0F\xB0`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x0E\xD7V[\x91P\x80\x82R\x8B\x84\x82\x85\x01\x01\x11\x15a\x0F\xC6W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\xFCW`\0\x80\xFD[\x835a\x10\x07\x81a\r\xDFV[\x92P` \x84\x015a\x10\x17\x81a\r\xDFV[\x91P`@\x84\x015a\x10'\x81a\r\xDFV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10GW`\0\x80\xFD[\x835a\x10R\x81a\r\xDFV[\x92P` \x84\x015a\x10b\x81a\r\xDFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x10\x85W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x0E!W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x10\xA8W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xC0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x10\xDBW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x10\xF5W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x0CW`\0\x80\xFD[a\x11\x18\x85\x82\x86\x01a\x10\x96V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x117W`\0\x80\xFD[\x825a\x11B\x81a\r\xDFV[\x91P` \x83\x015a\x11R\x81a\r\xDFV[\x80\x91PP\x92P\x92\x90PV[`@\x81R`\0a\x11p`@\x83\x01\x85a\x0E(V[\x82\x81\x03` \x84\x81\x01\x91\x90\x91R\x84Q\x80\x83R\x85\x82\x01\x92\x82\x01\x90`\0[\x81\x81\x10\x15a\x11\xA7W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x11\x8BV[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x11\xC7W`\0\x80\xFD[\x825a\x11\xD2\x81a\r\xDFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xF5W`\0\x80\xFD[\x835a\x12\0\x81a\r\xDFV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12/Wa\x12/a\x0E\x98V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x12JW`\0\x80\xFD[\x815` a\x12_a\x12Z\x83a\x12\x15V[a\x0E\xD7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x12~W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x12\xA2W\x805a\x12\x95\x81a\r\xDFV[\x83R\x91\x83\x01\x91\x83\x01a\x12\x82V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x12\xBEW`\0\x80\xFD[\x815` a\x12\xCEa\x12Z\x83a\x12\x15V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x12\xEDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x12\xA2W\x805\x83R\x91\x83\x01\x91\x83\x01a\x12\xF1V[`\0`@\x82\x84\x03\x12\x15a\x13\x1AW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13=Wa\x13=a\x0E\x98V[`@R\x90P\x80\x825a\x13N\x81a\r\xDFV[\x81R` \x83\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13oW`\0\x80\xFD[` \x91\x90\x91\x01R\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xFFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\xA2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\xBAW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x13\xCEW`\0\x80\xFD[a\x13\xD6a\x0E\xAEV[\x825\x82\x81\x11\x15a\x13\xE5W`\0\x80\xFD[a\x13\xF1\x87\x82\x86\x01a\x129V[\x82RP` \x83\x015\x82\x81\x11\x15a\x14\x06W`\0\x80\xFD[a\x14\x12\x87\x82\x86\x01a\x12\xADV[` \x83\x01RPa\x14$`@\x84\x01a\r\xF4V[`@\x82\x01Ra\x146\x86``\x85\x01a\x13\x08V[``\x82\x01Ra\x14G`\xA0\x84\x01a\x13|V[`\x80\x82\x01Ra\x14X`\xC0\x84\x01a\r\xF4V[`\xA0\x82\x01R\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14|W`\0\x80\xFD[\x845a\x14\x87\x81a\r\xDFV[\x93P` \x85\x015a\x14\x97\x81a\r\xDFV[\x92P`@\x85\x015\x91P``\x85\x015a\x14\xAE\x81a\r\xDFV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x14\xCFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xE7W`\0\x80\xFD[a\x14\xF3\x88\x83\x89\x01a\x10\x96V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x15\x0CW`\0\x80\xFD[Pa\x15\x19\x87\x82\x88\x01a\x10\x96V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\x157W`\0\x80\xFD[\x81Qa\x0E!\x81a\r\xDFV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x15\x9EW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E!W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V\xFE\xA2dipfsX\"\x12 \xE2\xF1\x02GU*\x7F\xC7\x86\xED\xA4\x15\xDF?mC\x86\x17\xE8\xF2\x03\xD6\xCB\xA4\xE3x#\xCA\xD4\xD9\xFD\xE1dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STRATEGYMANAGERMOCK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02'W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x010W\x80c\xB4;QK\x11a\0\xB8W\x80c\xCDGL\x8A\x11a\0|W\x80c\xCDGL\x8A\x14a\x04\xFAW\x80c\xDF\\\xF7#\x14a\x05\rW\x80c\xE7\xA0P\xAA\x14a\x05 W\x80c\xF2\xFD\xE3\x8B\x14a\x057W\x80c\xFA\xBC\x1C\xBC\x14a\x05JW`\0\x80\xFD[\x80c\xB4;QK\x14a\x04\x92W\x80c\xB5\xD8\xB5\xB8\x14a\x03yW\x80c\xC6\x08\xC7\xF3\x14a\x04\xA6W\x80c\xC7;B\xBE\x14a\x04\xBAW\x80c\xCD)?o\x14a\x04\xCDW`\0\x80\xFD[\x80c\x9A\x95\x19\xE0\x11a\0\xFFW\x80c\x9A\x95\x19\xE0\x14a\x040W\x80c\x9F\0\xFA$\x14a\x04CW\x80c\xA1x\x84\x84\x14a\x04QW\x80c\xA1\xCAx\x0B\x14a\x04qW\x80c\xB14Bq\x14a\x04\x7FW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x03\xE4W\x80c\x91\x04\xC3\x19\x14a\x03\xF5W\x80c\x94\xF6I\xDD\x14a\x03\xFCW\x80c\x96\x7F\xC0\xD2\x14a\x04\x1DW`\0\x80\xFD[\x80cZ\xC8j\xB7\x11a\x01\xB3W\x80cz~\r\x92\x11a\x01\x82W\x80cz~\r\x92\x14a\x03\x93W\x80cz\xB8\x01\xD1\x14a\x03\xA9W\x80c\x88o\x11\x95\x14a\x03\xBCW\x80c\x8B\x8A\xAC<\x14a\x03\xCFW\x80c\x8C\x80\xD4\xE5\x14a\x03#W`\0\x80\xFD[\x80cZ\xC8j\xB7\x14a\x03>W\x80c\\\x97Z\xBB\x14a\x03qW\x80c]\xE0\x8F\xF2\x14a\x03yW\x80cqP\x18\xA6\x14a\x03\x8BW`\0\x80\xFD[\x80c2\xE8\x9A\xCE\x11a\x01\xFAW\x80c2\xE8\x9A\xCE\x14a\x02\x91W\x80c6;\xF9d\x14a\x02\xABW\x80cFe\xBC\xDA\x14a\x02\xF8W\x80cP\xFFr%\x14a\x03#W\x80cY\\jg\x14a\x036W`\0\x80\xFD[\x80c\x01\xF8 \xB2\x14a\x02,W\x80c\r9\x08\xF4\x14a\x02HW\x80c\x10\xD6z/\x14a\x02iW\x80c\x13d9\xDD\x14a\x02~W[`\0\x80\xFD[a\x025`\xD0T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\\a\x02V6`\x04a\x0E\x04V[P``\x90V[`@Qa\x02?\x91\x90a\x0ElV[a\x02|a\x02w6`\x04a\x0E\x04V[a\x05]V[\0[a\x02|a\x02\x8C6`\x04a\x0E\x7FV[a\x06\x19V[a\x025a\x02\x9F6`\x04a\x0F\x08V[`\0\x96\x95PPPPPPV[a\x02|a\x02\xB96`\x04a\x0F\xE7V[`\xC9\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\xCB\x80T\x92\x85\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\xCA\x80T\x92\x90\x93\x16\x91\x16\x17\x90UV[`\xCATa\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02?V[a\x02|a\x0316`\x04a\x102V[PPPV[a\x02|a\x07XV[a\x03aa\x03L6`\x04a\x10sV[`\x98T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02?V[`\x98Ta\x025V[a\x02|a\x03\x876`\x04a\x10\xE2V[PPV[a\x02|a\x08\x1FV[a\x025a\x03\xA16`\x04a\x11$V[`\0\x92\x91PPV[a\x03\x0Ba\x03\xB76`\x04a\x0E\x7FV[a\x083V[`\x97Ta\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x025a\x03\xDD6`\x04a\x0E\x04V[P`\xD0T\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x0BV[`\0a\x03\x0BV[a\x04\x0Fa\x04\n6`\x04a\x0E\x04V[a\x08]V[`@Qa\x02?\x92\x91\x90a\x11]V[`\xCCTa\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02|a\x04>6`\x04a\x0E\x7FV[`\xD0UV[a\x02|a\x03\x876`\x04a\x11\xB4V[a\x025a\x04_6`\x04a\x0E\x04V[`\xCF` R`\0\x90\x81R`@\x90 T\x81V[a\x02|a\x0316`\x04a\x11\xE0V[`\xCBTa\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x025a\x04\xA06`\x04a\x13\x90V[P`\0\x90V[a\x02|a\x04\xB46`\x04a\x14fV[PPPPV[a\x02|a\x04\xC86`\x04a\x14\xB9V[a\t\x1BV[a\x04\xE3a\x04\xDB6`\x04a\x13\x90V[`\0\x80\x91P\x91V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02?V[a\x025a\x05\x086`\x04a\x0E\x7FV[a\t\x96V[`\xC9Ta\x03\x0B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x025a\x05.6`\x04a\x102V[`\0\x93\x92PPPV[a\x02|a\x05E6`\x04a\x0E\x04V[a\t\xB7V[a\x02|a\x05X6`\x04a\x0E\x7FV[a\n-V[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD4\x91\x90a\x15%V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15BV[`@Q\x80\x91\x03\x90\xFD[a\x06\x16\x81a\x0B\x89V[PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x85\x91\x90a\x15\x8CV[a\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15\xAEV[`\x98T\x81\x81\x16\x14a\x07\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x04V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xC4\x91\x90a\x15\x8CV[a\x07\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15\xAEV[`\0\x19`\x98\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x08'a\x0C\x80V[a\x081`\0a\x0C\xDAV[V[`\xCD\x81\x81T\x81\x10a\x08CW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[``\x80`\xCD`\xCE\x81\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xB9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x9BW[PPPPP\x91P\x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x0BW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xF7W[PPPPP\x90P\x91P\x91P\x91P\x91V[\x82\x81\x14a\tvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FStrategyManagerMock: length mism`D\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`d\x82\x01R`\x84\x01a\x06\x04V[a\t\x82`\xCD\x85\x85a\r,V[Pa\t\x8F`\xCE\x83\x83a\r\x8FV[PPPPPV[`\xCE\x81\x81T\x81\x10a\t\xA6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[a\t\xBFa\x0C\x80V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\x04V[a\x06\x16\x81a\x0C\xDAV[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA4\x91\x90a\x15%V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x04\x90a\x15BV[`\x98T\x19\x81\x19`\x98T\x19\x16\x14a\x0BRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x04V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07MV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\x04V[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x081W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\x04V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\r\x7FW\x91` \x02\x82\x01[\x82\x81\x11\x15a\r\x7FW\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x845\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\rLV[Pa\r\x8B\x92\x91Pa\r\xCAV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\r\x7FW\x91` \x02\x82\x01[\x82\x81\x11\x15a\r\x7FW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a\r\xAFV[[\x80\x82\x11\x15a\r\x8BW`\0\x81U`\x01\x01a\r\xCBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x16W`\0\x80\xFD[\x805a\r\xFF\x81a\r\xDFV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\x16W`\0\x80\xFD[\x815a\x0E!\x81a\r\xDFV[\x93\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x0EaW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0E<V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x0E!` \x83\x01\x84a\x0E(V[`\0` \x82\x84\x03\x12\x15a\x0E\x91W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xD1Wa\x0E\xD1a\x0E\x98V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\0Wa\x0F\0a\x0E\x98V[`@R\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0F!W`\0\x80\xFD[\x865a\x0F,\x81a\r\xDFV[\x95P` \x87\x81\x015a\x0F=\x81a\r\xDFV[\x95P`@\x88\x015\x94P``\x88\x015a\x0FT\x81a\r\xDFV[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FxW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x0F\x8CW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\x9EWa\x0F\x9Ea\x0E\x98V[a\x0F\xB0`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x0E\xD7V[\x91P\x80\x82R\x8B\x84\x82\x85\x01\x01\x11\x15a\x0F\xC6W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\xFCW`\0\x80\xFD[\x835a\x10\x07\x81a\r\xDFV[\x92P` \x84\x015a\x10\x17\x81a\r\xDFV[\x91P`@\x84\x015a\x10'\x81a\r\xDFV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10GW`\0\x80\xFD[\x835a\x10R\x81a\r\xDFV[\x92P` \x84\x015a\x10b\x81a\r\xDFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x10\x85W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x0E!W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x10\xA8W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xC0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x10\xDBW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x10\xF5W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x0CW`\0\x80\xFD[a\x11\x18\x85\x82\x86\x01a\x10\x96V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x117W`\0\x80\xFD[\x825a\x11B\x81a\r\xDFV[\x91P` \x83\x015a\x11R\x81a\r\xDFV[\x80\x91PP\x92P\x92\x90PV[`@\x81R`\0a\x11p`@\x83\x01\x85a\x0E(V[\x82\x81\x03` \x84\x81\x01\x91\x90\x91R\x84Q\x80\x83R\x85\x82\x01\x92\x82\x01\x90`\0[\x81\x81\x10\x15a\x11\xA7W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x11\x8BV[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x11\xC7W`\0\x80\xFD[\x825a\x11\xD2\x81a\r\xDFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xF5W`\0\x80\xFD[\x835a\x12\0\x81a\r\xDFV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12/Wa\x12/a\x0E\x98V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x12JW`\0\x80\xFD[\x815` a\x12_a\x12Z\x83a\x12\x15V[a\x0E\xD7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x12~W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x12\xA2W\x805a\x12\x95\x81a\r\xDFV[\x83R\x91\x83\x01\x91\x83\x01a\x12\x82V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x12\xBEW`\0\x80\xFD[\x815` a\x12\xCEa\x12Z\x83a\x12\x15V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x12\xEDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x12\xA2W\x805\x83R\x91\x83\x01\x91\x83\x01a\x12\xF1V[`\0`@\x82\x84\x03\x12\x15a\x13\x1AW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13=Wa\x13=a\x0E\x98V[`@R\x90P\x80\x825a\x13N\x81a\r\xDFV[\x81R` \x83\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13oW`\0\x80\xFD[` \x91\x90\x91\x01R\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xFFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\xA2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\xBAW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x13\xCEW`\0\x80\xFD[a\x13\xD6a\x0E\xAEV[\x825\x82\x81\x11\x15a\x13\xE5W`\0\x80\xFD[a\x13\xF1\x87\x82\x86\x01a\x129V[\x82RP` \x83\x015\x82\x81\x11\x15a\x14\x06W`\0\x80\xFD[a\x14\x12\x87\x82\x86\x01a\x12\xADV[` \x83\x01RPa\x14$`@\x84\x01a\r\xF4V[`@\x82\x01Ra\x146\x86``\x85\x01a\x13\x08V[``\x82\x01Ra\x14G`\xA0\x84\x01a\x13|V[`\x80\x82\x01Ra\x14X`\xC0\x84\x01a\r\xF4V[`\xA0\x82\x01R\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14|W`\0\x80\xFD[\x845a\x14\x87\x81a\r\xDFV[\x93P` \x85\x015a\x14\x97\x81a\r\xDFV[\x92P`@\x85\x015\x91P``\x85\x015a\x14\xAE\x81a\r\xDFV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x14\xCFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xE7W`\0\x80\xFD[a\x14\xF3\x88\x83\x89\x01a\x10\x96V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x15\x0CW`\0\x80\xFD[Pa\x15\x19\x87\x82\x88\x01a\x10\x96V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\x157W`\0\x80\xFD[\x81Qa\x0E!\x81a\r\xDFV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x15\x9EW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E!W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V\xFE\xA2dipfsX\"\x12 \xE2\xF1\x02GU*\x7F\xC7\x86\xED\xA4\x15\xDF?mC\x86\x17\xE8\xF2\x03\xD6\xCB\xA4\xE3x#\xCA\xD4\xD9\xFD\xE1dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static STRATEGYMANAGERMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StrategyManagerMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StrategyManagerMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StrategyManagerMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StrategyManagerMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StrategyManagerMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StrategyManagerMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StrategyManagerMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STRATEGYMANAGERMOCK_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                STRATEGYMANAGERMOCK_ABI.clone(),
                STRATEGYMANAGERMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addShares` (0x50ff7225) function
        pub fn add_shares(
            &self,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 255, 114, 37], (staker, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addStrategiesToDepositWhitelist` (0x5de08ff2) function
        pub fn add_strategies_to_deposit_whitelist(
            &self,
            p0: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 224, 143, 242], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beaconChainETHStrategy` (0x9104c319) function
        pub fn beacon_chain_eth_strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([145, 4, 195, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateWithdrawalRoot` (0xb43b514b) function
        pub fn calculate_withdrawal_root(
            &self,
            queued_withdrawal: DeprecatedStructQueuedWithdrawal,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([180, 59, 81, 75], (queued_withdrawal,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeWithdrawalsQueued` (0xa1788484) function
        pub fn cumulative_withdrawals_queued(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 120, 132, 132], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([223, 92, 247, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositBeaconChainETH` (0x9f00fa24) function
        pub fn deposit_beacon_chain_eth(
            &self,
            staker: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 0, 250, 36], (staker, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositIntoStrategy` (0xe7a050aa) function
        pub fn deposit_into_strategy(
            &self,
            strategy: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 160, 80, 170], (strategy, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositIntoStrategyWithSignature` (0x32e89ace) function
        pub fn deposit_into_strategy_with_signature(
            &self,
            strategy: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            staker: ::ethers::core::types::Address,
            expiry: ::ethers::core::types::U256,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [50, 232, 154, 206],
                    (strategy, token, amount, staker, expiry, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eigenPodManager` (0x4665bcda) function
        pub fn eigen_pod_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([70, 101, 188, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeposits` (0x94f649dd) function
        pub fn get_deposits(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([148, 246, 73, 221], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrateQueuedWithdrawal` (0xcd293f6f) function
        pub fn migrate_queued_withdrawal(
            &self,
            queued_withdrawal: DeprecatedStructQueuedWithdrawal,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, [u8; 32])> {
            self.0
                .method_hash([205, 41, 63, 111], (queued_withdrawal,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x136439dd) function
        pub fn pause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 100, 57, 221], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseAll` (0x595c6a67) function
        pub fn pause_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 92, 106, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5ac86ab7) function
        pub fn paused_with_index(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 200, 106, 183], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauserRegistry` (0x886f1195) function
        pub fn pauser_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordBeaconChainETHBalanceUpdate` (0xa1ca780b) function
        pub fn record_beacon_chain_eth_balance_update(
            &self,
            overcommitted_pod_owner: ::ethers::core::types::Address,
            beacon_chain_eth_strategy_index: ::ethers::core::types::U256,
            shares_delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [161, 202, 120, 11],
                    (
                        overcommitted_pod_owner,
                        beacon_chain_eth_strategy_index,
                        shares_delta,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeShares` (0x8c80d4e5) function
        pub fn remove_shares(
            &self,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 128, 212, 229], (staker, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStrategiesFromDepositWhitelist` (0xb5d8b5b8) function
        pub fn remove_strategies_from_deposit_whitelist(
            &self,
            p0: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 216, 181, 184], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAddresses` (0x363bf964) function
        pub fn set_addresses(
            &self,
            delegation: ::ethers::core::types::Address,
            eigen_pod_manager: ::ethers::core::types::Address,
            slasher: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 59, 249, 100], (delegation, eigen_pod_manager, slasher))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDeposits` (0xc73b42be) function
        pub fn set_deposits(
            &self,
            strategies_to_return: ::std::vec::Vec<::ethers::core::types::Address>,
            shares_to_return: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 59, 66, 190], (strategies_to_return, shares_to_return))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPauserRegistry` (0x10d67a2f) function
        pub fn set_pauser_registry(
            &self,
            new_pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 214, 122, 47], new_pauser_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStakerStrategyListLengthReturnValue` (0x9a9519e0) function
        pub fn set_staker_strategy_list_length_return_value(
            &self,
            value_to_set: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 149, 25, 224], value_to_set)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharesToReturn` (0xcd474c8a) function
        pub fn shares_to_return(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 71, 76, 138], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slasher` (0xb1344271) function
        pub fn slasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([177, 52, 66, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerStrategyListLength` (0x8b8aac3c) function
        pub fn staker_strategy_list_length(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 138, 172, 60], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerStrategyListLengthReturnValue` (0x01f820b2) function
        pub fn staker_strategy_list_length_return_value(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 248, 32, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerStrategyShares` (0x7a7e0d92) function
        pub fn staker_strategy_shares(
            &self,
            user: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 126, 13, 146], (user, strategy))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerStrats` (0x0d3908f4) function
        pub fn staker_strats(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([13, 57, 8, 244], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategiesToReturn` (0x7ab801d1) function
        pub fn strategies_to_return(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([122, 184, 1, 209], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyWhitelister` (0x967fc0d2) function
        pub fn strategy_whitelister(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([150, 127, 192, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawSharesAsTokens` (0xc608c7f3) function
        pub fn withdraw_shares_as_tokens(
            &self,
            recipient: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 8, 199, 243], (recipient, strategy, shares, token))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PauserRegistrySet` event
        pub fn pauser_registry_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauserRegistrySetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StrategyAddedToDepositWhitelist` event
        pub fn strategy_added_to_deposit_whitelist_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyAddedToDepositWhitelistFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyRemovedFromDepositWhitelist` event
        pub fn strategy_removed_from_deposit_whitelist_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyRemovedFromDepositWhitelistFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyWhitelisterChanged` event
        pub fn strategy_whitelister_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyWhitelisterChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StrategyManagerMockEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for StrategyManagerMock<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,address,uint256)")]
    pub struct DepositFilter {
        pub staker: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Paused", abi = "Paused(address,uint256)")]
    pub struct PausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PauserRegistrySet", abi = "PauserRegistrySet(address,address)")]
    pub struct PauserRegistrySetFilter {
        pub pauser_registry: ::ethers::core::types::Address,
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "StrategyAddedToDepositWhitelist",
        abi = "StrategyAddedToDepositWhitelist(address)"
    )]
    pub struct StrategyAddedToDepositWhitelistFilter {
        pub strategy: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "StrategyRemovedFromDepositWhitelist",
        abi = "StrategyRemovedFromDepositWhitelist(address)"
    )]
    pub struct StrategyRemovedFromDepositWhitelistFilter {
        pub strategy: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "StrategyWhitelisterChanged",
        abi = "StrategyWhitelisterChanged(address,address)"
    )]
    pub struct StrategyWhitelisterChangedFilter {
        pub previous_address: ::ethers::core::types::Address,
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address,uint256)")]
    pub struct UnpausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum StrategyManagerMockEvents {
        DepositFilter(DepositFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        StrategyAddedToDepositWhitelistFilter(StrategyAddedToDepositWhitelistFilter),
        StrategyRemovedFromDepositWhitelistFilter(StrategyRemovedFromDepositWhitelistFilter),
        StrategyWhitelisterChangedFilter(StrategyWhitelisterChangedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for StrategyManagerMockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(StrategyManagerMockEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StrategyManagerMockEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(StrategyManagerMockEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(StrategyManagerMockEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(StrategyManagerMockEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = StrategyAddedToDepositWhitelistFilter::decode_log(log) {
                return Ok(
                    StrategyManagerMockEvents::StrategyAddedToDepositWhitelistFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyRemovedFromDepositWhitelistFilter::decode_log(log) {
                return Ok(
                    StrategyManagerMockEvents::StrategyRemovedFromDepositWhitelistFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyWhitelisterChangedFilter::decode_log(log) {
                return Ok(StrategyManagerMockEvents::StrategyWhitelisterChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(StrategyManagerMockEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StrategyManagerMockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyAddedToDepositWhitelistFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyRemovedFromDepositWhitelistFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyWhitelisterChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositFilter> for StrategyManagerMockEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for StrategyManagerMockEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for StrategyManagerMockEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for StrategyManagerMockEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for StrategyManagerMockEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<StrategyAddedToDepositWhitelistFilter> for StrategyManagerMockEvents {
        fn from(value: StrategyAddedToDepositWhitelistFilter) -> Self {
            Self::StrategyAddedToDepositWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<StrategyRemovedFromDepositWhitelistFilter>
        for StrategyManagerMockEvents
    {
        fn from(value: StrategyRemovedFromDepositWhitelistFilter) -> Self {
            Self::StrategyRemovedFromDepositWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<StrategyWhitelisterChangedFilter> for StrategyManagerMockEvents {
        fn from(value: StrategyWhitelisterChangedFilter) -> Self {
            Self::StrategyWhitelisterChangedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for StrategyManagerMockEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addShares` function with signature `addShares(address,address,uint256)` and selector `0x50ff7225`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addShares", abi = "addShares(address,address,uint256)")]
    pub struct AddSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addStrategiesToDepositWhitelist` function with signature `addStrategiesToDepositWhitelist(address[])` and selector `0x5de08ff2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addStrategiesToDepositWhitelist",
        abi = "addStrategiesToDepositWhitelist(address[])"
    )]
    pub struct AddStrategiesToDepositWhitelistCall(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all input parameters for the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "beaconChainETHStrategy", abi = "beaconChainETHStrategy()")]
    pub struct BeaconChainETHStrategyCall;
    ///Container type for all input parameters for the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xb43b514b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "calculateWithdrawalRoot",
        abi = "calculateWithdrawalRoot((address[],uint256[],address,(address,uint96),uint32,address))"
    )]
    pub struct CalculateWithdrawalRootCall {
        pub queued_withdrawal: DeprecatedStructQueuedWithdrawal,
    }
    ///Container type for all input parameters for the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "cumulativeWithdrawalsQueued",
        abi = "cumulativeWithdrawalsQueued(address)"
    )]
    pub struct CumulativeWithdrawalsQueuedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
    ///Container type for all input parameters for the `depositBeaconChainETH` function with signature `depositBeaconChainETH(address,uint256)` and selector `0x9f00fa24`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "depositBeaconChainETH",
        abi = "depositBeaconChainETH(address,uint256)"
    )]
    pub struct DepositBeaconChainETHCall {
        pub staker: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `depositIntoStrategy` function with signature `depositIntoStrategy(address,address,uint256)` and selector `0xe7a050aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "depositIntoStrategy",
        abi = "depositIntoStrategy(address,address,uint256)"
    )]
    pub struct DepositIntoStrategyCall {
        pub strategy: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `depositIntoStrategyWithSignature` function with signature `depositIntoStrategyWithSignature(address,address,uint256,address,uint256,bytes)` and selector `0x32e89ace`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "depositIntoStrategyWithSignature",
        abi = "depositIntoStrategyWithSignature(address,address,uint256,address,uint256,bytes)"
    )]
    pub struct DepositIntoStrategyWithSignatureCall {
        pub strategy: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub staker: ::ethers::core::types::Address,
        pub expiry: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "eigenPodManager", abi = "eigenPodManager()")]
    pub struct EigenPodManagerCall;
    ///Container type for all input parameters for the `getDeposits` function with signature `getDeposits(address)` and selector `0x94f649dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getDeposits", abi = "getDeposits(address)")]
    pub struct GetDepositsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `migrateQueuedWithdrawal` function with signature `migrateQueuedWithdrawal((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xcd293f6f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "migrateQueuedWithdrawal",
        abi = "migrateQueuedWithdrawal((address[],uint256[],address,(address,uint96),uint32,address))"
    )]
    pub struct MigrateQueuedWithdrawalCall {
        pub queued_withdrawal: DeprecatedStructQueuedWithdrawal,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause(uint256)` and selector `0x136439dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pause", abi = "pause(uint256)")]
    pub struct PauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pauseAll` function with signature `pauseAll()` and selector `0x595c6a67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pauseAll", abi = "pauseAll()")]
    pub struct PauseAllCall;
    ///Container type for all input parameters for the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "paused", abi = "paused(uint8)")]
    pub struct PausedWithIndexCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `recordBeaconChainETHBalanceUpdate` function with signature `recordBeaconChainETHBalanceUpdate(address,uint256,int256)` and selector `0xa1ca780b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "recordBeaconChainETHBalanceUpdate",
        abi = "recordBeaconChainETHBalanceUpdate(address,uint256,int256)"
    )]
    pub struct RecordBeaconChainETHBalanceUpdateCall {
        pub overcommitted_pod_owner: ::ethers::core::types::Address,
        pub beacon_chain_eth_strategy_index: ::ethers::core::types::U256,
        pub shares_delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `removeShares` function with signature `removeShares(address,address,uint256)` and selector `0x8c80d4e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "removeShares", abi = "removeShares(address,address,uint256)")]
    pub struct RemoveSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeStrategiesFromDepositWhitelist` function with signature `removeStrategiesFromDepositWhitelist(address[])` and selector `0xb5d8b5b8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "removeStrategiesFromDepositWhitelist",
        abi = "removeStrategiesFromDepositWhitelist(address[])"
    )]
    pub struct RemoveStrategiesFromDepositWhitelistCall(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setAddresses` function with signature `setAddresses(address,address,address)` and selector `0x363bf964`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setAddresses", abi = "setAddresses(address,address,address)")]
    pub struct SetAddressesCall {
        pub delegation: ::ethers::core::types::Address,
        pub eigen_pod_manager: ::ethers::core::types::Address,
        pub slasher: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setDeposits` function with signature `setDeposits(address[],uint256[])` and selector `0xc73b42be`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setDeposits", abi = "setDeposits(address[],uint256[])")]
    pub struct SetDepositsCall {
        pub strategies_to_return: ::std::vec::Vec<::ethers::core::types::Address>,
        pub shares_to_return: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `setPauserRegistry` function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStakerStrategyListLengthReturnValue` function with signature `setStakerStrategyListLengthReturnValue(uint256)` and selector `0x9a9519e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setStakerStrategyListLengthReturnValue",
        abi = "setStakerStrategyListLengthReturnValue(uint256)"
    )]
    pub struct SetStakerStrategyListLengthReturnValueCall {
        pub value_to_set: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sharesToReturn` function with signature `sharesToReturn(uint256)` and selector `0xcd474c8a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "sharesToReturn", abi = "sharesToReturn(uint256)")]
    pub struct SharesToReturnCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `slasher` function with signature `slasher()` and selector `0xb1344271`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "slasher", abi = "slasher()")]
    pub struct SlasherCall;
    ///Container type for all input parameters for the `stakerStrategyListLength` function with signature `stakerStrategyListLength(address)` and selector `0x8b8aac3c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "stakerStrategyListLength",
        abi = "stakerStrategyListLength(address)"
    )]
    pub struct StakerStrategyListLengthCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `stakerStrategyListLengthReturnValue` function with signature `stakerStrategyListLengthReturnValue()` and selector `0x01f820b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "stakerStrategyListLengthReturnValue",
        abi = "stakerStrategyListLengthReturnValue()"
    )]
    pub struct StakerStrategyListLengthReturnValueCall;
    ///Container type for all input parameters for the `stakerStrategyShares` function with signature `stakerStrategyShares(address,address)` and selector `0x7a7e0d92`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "stakerStrategyShares",
        abi = "stakerStrategyShares(address,address)"
    )]
    pub struct StakerStrategySharesCall {
        pub user: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stakerStrats` function with signature `stakerStrats(address)` and selector `0x0d3908f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "stakerStrats", abi = "stakerStrats(address)")]
    pub struct StakerStratsCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `strategiesToReturn` function with signature `strategiesToReturn(uint256)` and selector `0x7ab801d1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "strategiesToReturn", abi = "strategiesToReturn(uint256)")]
    pub struct StrategiesToReturnCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `strategyWhitelister` function with signature `strategyWhitelister()` and selector `0x967fc0d2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "strategyWhitelister", abi = "strategyWhitelister()")]
    pub struct StrategyWhitelisterCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause(uint256)` and selector `0xfabc1cbc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unpause", abi = "unpause(uint256)")]
    pub struct UnpauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawSharesAsTokens` function with signature `withdrawSharesAsTokens(address,address,uint256,address)` and selector `0xc608c7f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "withdrawSharesAsTokens",
        abi = "withdrawSharesAsTokens(address,address,uint256,address)"
    )]
    pub struct WithdrawSharesAsTokensCall {
        pub recipient: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum StrategyManagerMockCalls {
        AddShares(AddSharesCall),
        AddStrategiesToDepositWhitelist(AddStrategiesToDepositWhitelistCall),
        BeaconChainETHStrategy(BeaconChainETHStrategyCall),
        CalculateWithdrawalRoot(CalculateWithdrawalRootCall),
        CumulativeWithdrawalsQueued(CumulativeWithdrawalsQueuedCall),
        Delegation(DelegationCall),
        DepositBeaconChainETH(DepositBeaconChainETHCall),
        DepositIntoStrategy(DepositIntoStrategyCall),
        DepositIntoStrategyWithSignature(DepositIntoStrategyWithSignatureCall),
        EigenPodManager(EigenPodManagerCall),
        GetDeposits(GetDepositsCall),
        MigrateQueuedWithdrawal(MigrateQueuedWithdrawalCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RecordBeaconChainETHBalanceUpdate(RecordBeaconChainETHBalanceUpdateCall),
        RemoveShares(RemoveSharesCall),
        RemoveStrategiesFromDepositWhitelist(RemoveStrategiesFromDepositWhitelistCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetAddresses(SetAddressesCall),
        SetDeposits(SetDepositsCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetStakerStrategyListLengthReturnValue(SetStakerStrategyListLengthReturnValueCall),
        SharesToReturn(SharesToReturnCall),
        Slasher(SlasherCall),
        StakerStrategyListLength(StakerStrategyListLengthCall),
        StakerStrategyListLengthReturnValue(StakerStrategyListLengthReturnValueCall),
        StakerStrategyShares(StakerStrategySharesCall),
        StakerStrats(StakerStratsCall),
        StrategiesToReturn(StrategiesToReturnCall),
        StrategyWhitelister(StrategyWhitelisterCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        WithdrawSharesAsTokens(WithdrawSharesAsTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for StrategyManagerMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddSharesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddShares(decoded));
            }
            if let Ok(decoded) =
                <AddStrategiesToDepositWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::AddStrategiesToDepositWhitelist(decoded));
            }
            if let Ok(decoded) =
                <BeaconChainETHStrategyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BeaconChainETHStrategy(decoded));
            }
            if let Ok(decoded) =
                <CalculateWithdrawalRootCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CalculateWithdrawalRoot(decoded));
            }
            if let Ok(decoded) =
                <CumulativeWithdrawalsQueuedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeWithdrawalsQueued(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) =
                <DepositBeaconChainETHCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositBeaconChainETH(decoded));
            }
            if let Ok(decoded) =
                <DepositIntoStrategyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositIntoStrategy(decoded));
            }
            if let Ok(decoded) =
                <DepositIntoStrategyWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DepositIntoStrategyWithSignature(decoded));
            }
            if let Ok(decoded) =
                <EigenPodManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EigenPodManager(decoded));
            }
            if let Ok(decoded) = <GetDepositsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDeposits(decoded));
            }
            if let Ok(decoded) =
                <MigrateQueuedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MigrateQueuedWithdrawal(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) =
                <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) =
                <RecordBeaconChainETHBalanceUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RecordBeaconChainETHBalanceUpdate(decoded));
            }
            if let Ok(decoded) = <RemoveSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveShares(decoded));
            }
            if let Ok(decoded) =
                <RemoveStrategiesFromDepositWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RemoveStrategiesFromDepositWhitelist(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetAddressesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAddresses(decoded));
            }
            if let Ok(decoded) = <SetDepositsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDeposits(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetStakerStrategyListLengthReturnValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStakerStrategyListLengthReturnValue(decoded));
            }
            if let Ok(decoded) =
                <SharesToReturnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SharesToReturn(decoded));
            }
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) =
                <StakerStrategyListLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerStrategyListLength(decoded));
            }
            if let Ok(decoded) =
                <StakerStrategyListLengthReturnValueCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::StakerStrategyListLengthReturnValue(decoded));
            }
            if let Ok(decoded) =
                <StakerStrategySharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerStrategyShares(decoded));
            }
            if let Ok(decoded) = <StakerStratsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerStrats(decoded));
            }
            if let Ok(decoded) =
                <StrategiesToReturnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategiesToReturn(decoded));
            }
            if let Ok(decoded) =
                <StrategyWhitelisterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyWhitelister(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <WithdrawSharesAsTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawSharesAsTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StrategyManagerMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddStrategiesToDepositWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeaconChainETHStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CumulativeWithdrawalsQueued(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositBeaconChainETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositIntoStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositIntoStrategyWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EigenPodManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDeposits(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MigrateQueuedWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecordBeaconChainETHBalanceUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveStrategiesFromDepositWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAddresses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDeposits(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetStakerStrategyListLengthReturnValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SharesToReturn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakerStrategyListLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerStrategyListLengthReturnValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerStrategyShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerStrats(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrategiesToReturn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyWhitelister(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawSharesAsTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StrategyManagerMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStrategiesToDepositWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconChainETHStrategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateWithdrawalRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeWithdrawalsQueued(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositBeaconChainETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositIntoStrategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositIntoStrategyWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenPodManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrateQueuedWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordBeaconChainETHBalanceUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStrategiesFromDepositWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDeposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStakerStrategyListLengthReturnValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SharesToReturn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerStrategyListLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerStrategyListLengthReturnValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerStrategyShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerStrats(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategiesToReturn(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyWhitelister(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawSharesAsTokens(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddSharesCall> for StrategyManagerMockCalls {
        fn from(value: AddSharesCall) -> Self {
            Self::AddShares(value)
        }
    }
    impl ::core::convert::From<AddStrategiesToDepositWhitelistCall> for StrategyManagerMockCalls {
        fn from(value: AddStrategiesToDepositWhitelistCall) -> Self {
            Self::AddStrategiesToDepositWhitelist(value)
        }
    }
    impl ::core::convert::From<BeaconChainETHStrategyCall> for StrategyManagerMockCalls {
        fn from(value: BeaconChainETHStrategyCall) -> Self {
            Self::BeaconChainETHStrategy(value)
        }
    }
    impl ::core::convert::From<CalculateWithdrawalRootCall> for StrategyManagerMockCalls {
        fn from(value: CalculateWithdrawalRootCall) -> Self {
            Self::CalculateWithdrawalRoot(value)
        }
    }
    impl ::core::convert::From<CumulativeWithdrawalsQueuedCall> for StrategyManagerMockCalls {
        fn from(value: CumulativeWithdrawalsQueuedCall) -> Self {
            Self::CumulativeWithdrawalsQueued(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for StrategyManagerMockCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DepositBeaconChainETHCall> for StrategyManagerMockCalls {
        fn from(value: DepositBeaconChainETHCall) -> Self {
            Self::DepositBeaconChainETH(value)
        }
    }
    impl ::core::convert::From<DepositIntoStrategyCall> for StrategyManagerMockCalls {
        fn from(value: DepositIntoStrategyCall) -> Self {
            Self::DepositIntoStrategy(value)
        }
    }
    impl ::core::convert::From<DepositIntoStrategyWithSignatureCall> for StrategyManagerMockCalls {
        fn from(value: DepositIntoStrategyWithSignatureCall) -> Self {
            Self::DepositIntoStrategyWithSignature(value)
        }
    }
    impl ::core::convert::From<EigenPodManagerCall> for StrategyManagerMockCalls {
        fn from(value: EigenPodManagerCall) -> Self {
            Self::EigenPodManager(value)
        }
    }
    impl ::core::convert::From<GetDepositsCall> for StrategyManagerMockCalls {
        fn from(value: GetDepositsCall) -> Self {
            Self::GetDeposits(value)
        }
    }
    impl ::core::convert::From<MigrateQueuedWithdrawalCall> for StrategyManagerMockCalls {
        fn from(value: MigrateQueuedWithdrawalCall) -> Self {
            Self::MigrateQueuedWithdrawal(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for StrategyManagerMockCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for StrategyManagerMockCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for StrategyManagerMockCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for StrategyManagerMockCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for StrategyManagerMockCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for StrategyManagerMockCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RecordBeaconChainETHBalanceUpdateCall> for StrategyManagerMockCalls {
        fn from(value: RecordBeaconChainETHBalanceUpdateCall) -> Self {
            Self::RecordBeaconChainETHBalanceUpdate(value)
        }
    }
    impl ::core::convert::From<RemoveSharesCall> for StrategyManagerMockCalls {
        fn from(value: RemoveSharesCall) -> Self {
            Self::RemoveShares(value)
        }
    }
    impl ::core::convert::From<RemoveStrategiesFromDepositWhitelistCall> for StrategyManagerMockCalls {
        fn from(value: RemoveStrategiesFromDepositWhitelistCall) -> Self {
            Self::RemoveStrategiesFromDepositWhitelist(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for StrategyManagerMockCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetAddressesCall> for StrategyManagerMockCalls {
        fn from(value: SetAddressesCall) -> Self {
            Self::SetAddresses(value)
        }
    }
    impl ::core::convert::From<SetDepositsCall> for StrategyManagerMockCalls {
        fn from(value: SetDepositsCall) -> Self {
            Self::SetDeposits(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for StrategyManagerMockCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetStakerStrategyListLengthReturnValueCall>
        for StrategyManagerMockCalls
    {
        fn from(value: SetStakerStrategyListLengthReturnValueCall) -> Self {
            Self::SetStakerStrategyListLengthReturnValue(value)
        }
    }
    impl ::core::convert::From<SharesToReturnCall> for StrategyManagerMockCalls {
        fn from(value: SharesToReturnCall) -> Self {
            Self::SharesToReturn(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for StrategyManagerMockCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakerStrategyListLengthCall> for StrategyManagerMockCalls {
        fn from(value: StakerStrategyListLengthCall) -> Self {
            Self::StakerStrategyListLength(value)
        }
    }
    impl ::core::convert::From<StakerStrategyListLengthReturnValueCall> for StrategyManagerMockCalls {
        fn from(value: StakerStrategyListLengthReturnValueCall) -> Self {
            Self::StakerStrategyListLengthReturnValue(value)
        }
    }
    impl ::core::convert::From<StakerStrategySharesCall> for StrategyManagerMockCalls {
        fn from(value: StakerStrategySharesCall) -> Self {
            Self::StakerStrategyShares(value)
        }
    }
    impl ::core::convert::From<StakerStratsCall> for StrategyManagerMockCalls {
        fn from(value: StakerStratsCall) -> Self {
            Self::StakerStrats(value)
        }
    }
    impl ::core::convert::From<StrategiesToReturnCall> for StrategyManagerMockCalls {
        fn from(value: StrategiesToReturnCall) -> Self {
            Self::StrategiesToReturn(value)
        }
    }
    impl ::core::convert::From<StrategyWhitelisterCall> for StrategyManagerMockCalls {
        fn from(value: StrategyWhitelisterCall) -> Self {
            Self::StrategyWhitelister(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for StrategyManagerMockCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for StrategyManagerMockCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<WithdrawSharesAsTokensCall> for StrategyManagerMockCalls {
        fn from(value: WithdrawSharesAsTokensCall) -> Self {
            Self::WithdrawSharesAsTokens(value)
        }
    }
    ///Container type for all return fields from the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BeaconChainETHStrategyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xb43b514b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CalculateWithdrawalRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CumulativeWithdrawalsQueuedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DelegationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `depositIntoStrategy` function with signature `depositIntoStrategy(address,address,uint256)` and selector `0xe7a050aa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositIntoStrategyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `depositIntoStrategyWithSignature` function with signature `depositIntoStrategyWithSignature(address,address,uint256,address,uint256,bytes)` and selector `0x32e89ace`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositIntoStrategyWithSignatureReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EigenPodManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDeposits` function with signature `getDeposits(address)` and selector `0x94f649dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetDepositsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `migrateQueuedWithdrawal` function with signature `migrateQueuedWithdrawal((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xcd293f6f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MigrateQueuedWithdrawalReturn(pub bool, pub [u8; 32]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PausedWithIndexReturn(pub bool);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PausedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PauserRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `sharesToReturn` function with signature `sharesToReturn(uint256)` and selector `0xcd474c8a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SharesToReturnReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `slasher` function with signature `slasher()` and selector `0xb1344271`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SlasherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakerStrategyListLength` function with signature `stakerStrategyListLength(address)` and selector `0x8b8aac3c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StakerStrategyListLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakerStrategyListLengthReturnValue` function with signature `stakerStrategyListLengthReturnValue()` and selector `0x01f820b2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StakerStrategyListLengthReturnValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakerStrategyShares` function with signature `stakerStrategyShares(address,address)` and selector `0x7a7e0d92`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StakerStrategySharesReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `stakerStrats` function with signature `stakerStrats(address)` and selector `0x0d3908f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StakerStratsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `strategiesToReturn` function with signature `strategiesToReturn(uint256)` and selector `0x7ab801d1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StrategiesToReturnReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `strategyWhitelister` function with signature `strategyWhitelister()` and selector `0x967fc0d2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StrategyWhitelisterReturn(pub ::ethers::core::types::Address);
}
