// TODO to compute -- all initialized to ONE

use super::super::super::ge::GePrecomp;
use super::Fe;

#[rustfmt::skip]
pub(crate) const BI: [GePrecomp; 8] = [
    GePrecomp {
        y_plus_x: Fe([ 0x493c6f58c3b85, 0xdf7181c325f7, 0xf50b0b3e4cb7, 0x5329385a44c32, 0x7cf9d3a33d4b ]),
        y_minus_x: Fe([ 0x3905d740913e, 0xba2817d673a2, 0x23e2827f4e67c, 0x133d2e0c21a34, 0x44fd2f9298f81 ]),
        xy2d: Fe([ 0x11205877aaa68, 0x479955893d579, 0x50d66309b67a0, 0x2d42d0dbee5ee, 0x6f117b689f0c6 ]),
    },
    GePrecomp {
        y_plus_x: Fe([ 0x5b0a84cee9730, 0x61d10c97155e4, 0x4059cc8096a10, 0x47a608da8014f, 0x7a164e1b9a80f ]),
        y_minus_x: Fe([ 0x11fe8a4fcd265, 0x7bcb8374faacc, 0x52f5af4ef4d4f, 0x5314098f98d10, 0x2ab91587555bd ]),
        xy2d: Fe([ 0x6933f0dd0d889, 0x44386bb4c4295, 0x3cb6d3162508c, 0x26368b872a2c6, 0x5a2826af12b9b ]),
    },
    GePrecomp {
        y_plus_x: Fe([ 0x2bc4408a5bb33, 0x78ebdda05442, 0x2ffb112354123, 0x375ee8df5862d, 0x2945ccf146e20 ]),
        y_minus_x: Fe([ 0x182c3a447d6ba, 0x22964e536eff2, 0x192821f540053, 0x2f9f19e788e5c, 0x154a7e73eb1b5 ]),
        xy2d: Fe([ 0x3dbf1812a8285, 0xfa17ba3f9797, 0x6f69cb49c3820, 0x34d5a0db3858d, 0x43aabe696b3bb ]),
    },
    GePrecomp {
        y_plus_x: Fe([ 0x25cd0944ea3bf, 0x75673b81a4d63, 0x150b925d1c0d4, 0x13f38d9294114, 0x461bea69283c9 ]),
        y_minus_x: Fe([ 0x72c9aaa3221b1, 0x267774474f74d, 0x64b0e9b28085, 0x3f04ef53b27c9, 0x1d6edd5d2e531 ]),
        xy2d: Fe([ 0x36dc801b8b3a2, 0xe0a7d4935e30, 0x1deb7cecc0d7d, 0x53a94e20dd2c, 0x7a9fbb1c6a0f9 ]),
    },
    GePrecomp {
        y_plus_x: Fe([ 0x6678aa6a8632f, 0x5ea3788d8b365, 0x21bd6d6994279, 0x7ace75919e4e3, 0x34b9ed338add7 ]),
        y_minus_x: Fe([ 0x6217e039d8064, 0x6dea408337e6d, 0x57ac112628206, 0x647cb65e30473, 0x49c05a51fadc9 ]),
        xy2d: Fe([ 0x4e8bf9045af1b, 0x514e33a45e0d6, 0x7533c5b8bfe0f, 0x583557b7e14c9, 0x73c172021b008 ]),
    },
    GePrecomp {
        y_plus_x: Fe([ 0x700848a802ade, 0x1e04605c4e5f7, 0x5c0d01b9767fb, 0x7d7889f42388b, 0x4275aae2546d8 ]),
        y_minus_x: Fe([ 0x75b0249864348, 0x52ee11070262b, 0x237ae54fb5acd, 0x3bfd1d03aaab5, 0x18ab598029d5c ]),
        xy2d: Fe([ 0x32cc5fd6089e9, 0x426505c949b05, 0x46a18880c7ad2, 0x4a4221888ccda, 0x3dc65522b53df ]),
    },
    GePrecomp {
        y_plus_x: Fe([ 0xc222a2007f6d, 0x356b79bdb77ee, 0x41ee81efe12ce, 0x120a9bd07097d, 0x234fd7eec346f ]),
        y_minus_x: Fe([ 0x7013b327fbf93, 0x1336eeded6a0d, 0x2b565a2bbf3af, 0x253ce89591955, 0x267882d17602 ]),
        xy2d: Fe([ 0xa119732ea378, 0x63bf1ba8e2a6c, 0x69f94cc90df9a, 0x431d1779bfc48, 0x497ba6fdaa097 ]),
    },
    GePrecomp {
        y_plus_x: Fe([ 0x6cc0313cfeaa0, 0x1a313848da499, 0x7cb534219230a, 0x39596dedefd60, 0x61e22917f12de ]),
        y_minus_x: Fe([ 0x3cd86468ccf0b, 0x48553221ac081, 0x6c9464b4e0a6e, 0x75fba84180403, 0x43b5cd4218d05 ]),
        xy2d: Fe([ 0x2762f9bd0b516, 0x1c6e7fbddcbb3, 0x75909c3ace2bd, 0x42101972d3ec9, 0x511d61210ae4d ]),
    },
];

#[rustfmt::skip]
pub(crate) const GE_BASE: [[GePrecomp; 8]; 32] = [
    [
        GePrecomp {
            y_plus_x: Fe([ 0x493c6f58c3b85, 0xdf7181c325f7, 0xf50b0b3e4cb7, 0x5329385a44c32, 0x7cf9d3a33d4b ]),
            y_minus_x: Fe([ 0x3905d740913e, 0xba2817d673a2, 0x23e2827f4e67c, 0x133d2e0c21a34, 0x44fd2f9298f81 ]),
            xy2d: Fe([ 0x11205877aaa68, 0x479955893d579, 0x50d66309b67a0, 0x2d42d0dbee5ee, 0x6f117b689f0c6 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4e7fc933c71d7, 0x2cf41feb6b244, 0x7581c0a7d1a76, 0x7172d534d32f0, 0x590c063fa87d2 ]),
            y_minus_x: Fe([ 0x1a56042b4d5a8, 0x189cc159ed153, 0x5b8deaa3cae04, 0x2aaf04f11b5d8, 0x6bb595a669c92 ]),
            xy2d: Fe([ 0x2a8b3a59b7a5f, 0x3abb359ef087f, 0x4f5a8c4db05af, 0x5b9a807d04205, 0x701af5b13ea50 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5b0a84cee9730, 0x61d10c97155e4, 0x4059cc8096a10, 0x47a608da8014f, 0x7a164e1b9a80f ]),
            y_minus_x: Fe([ 0x11fe8a4fcd265, 0x7bcb8374faacc, 0x52f5af4ef4d4f, 0x5314098f98d10, 0x2ab91587555bd ]),
            xy2d: Fe([ 0x6933f0dd0d889, 0x44386bb4c4295, 0x3cb6d3162508c, 0x26368b872a2c6, 0x5a2826af12b9b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x351b98efc099f, 0x68fbfa4a7050e, 0x42a49959d971b, 0x393e51a469efd, 0x680e910321e58 ]),
            y_minus_x: Fe([ 0x6050a056818bf, 0x62acc1f5532bf, 0x28141ccc9fa25, 0x24d61f471e683, 0x27933f4c7445a ]),
            xy2d: Fe([ 0x3fbe9c476ff09, 0xaf6b982e4b42, 0xad1251ba78e5, 0x715aeedee7c88, 0x7f9d0cbf63553 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2bc4408a5bb33, 0x78ebdda05442, 0x2ffb112354123, 0x375ee8df5862d, 0x2945ccf146e20 ]),
            y_minus_x: Fe([ 0x182c3a447d6ba, 0x22964e536eff2, 0x192821f540053, 0x2f9f19e788e5c, 0x154a7e73eb1b5 ]),
            xy2d: Fe([ 0x3dbf1812a8285, 0xfa17ba3f9797, 0x6f69cb49c3820, 0x34d5a0db3858d, 0x43aabe696b3bb ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4eeeb77157131, 0x1201915f10741, 0x1669cda6c9c56, 0x45ec032db346d, 0x51e57bb6a2cc3 ]),
            y_minus_x: Fe([ 0x6b67b7d8ca4, 0x84fa44e72933, 0x1154ee55d6f8a, 0x4425d842e7390, 0x38b64c41ae417 ]),
            xy2d: Fe([ 0x4326702ea4b71, 0x6834376030b5, 0xef0512f9c380, 0xf1a9f2512584, 0x10b8e91a9f0d6 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x25cd0944ea3bf, 0x75673b81a4d63, 0x150b925d1c0d4, 0x13f38d9294114, 0x461bea69283c9 ]),
            y_minus_x: Fe([ 0x72c9aaa3221b1, 0x267774474f74d, 0x64b0e9b28085, 0x3f04ef53b27c9, 0x1d6edd5d2e531 ]),
            xy2d: Fe([ 0x36dc801b8b3a2, 0xe0a7d4935e30, 0x1deb7cecc0d7d, 0x53a94e20dd2c, 0x7a9fbb1c6a0f9 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7596604dd3e8f, 0x6fc510e058b36, 0x3670c8db2cc0d, 0x297d899ce332f, 0x915e76061bce ]),
            y_minus_x: Fe([ 0x75dedf39234d9, 0x1c36ab1f3c54, 0xf08fee58f5da, 0xe19613a0d637, 0x3a9024a1320e0 ]),
            xy2d: Fe([ 0x1f5d9c9a2911a, 0x7117994fafcf8, 0x2d8a8cae28dc5, 0x74ab1b2090c87, 0x26907c5c2ecc4 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x4dd0e632f9c1d, 0x2ced12622a5d9, 0x18de9614742da, 0x79ca96fdbb5d4, 0x6dd37d49a00ee ]),
            y_minus_x: Fe([ 0x3635449aa515e, 0x3e178d0475dab, 0x50b4712a19712, 0x2dcc2860ff4ad, 0x30d76d6f03d31 ]),
            xy2d: Fe([ 0x444172106e4c7, 0x1251afed2d88, 0x534fc9bed4f5a, 0x5d85a39cf5234, 0x10c697112e864 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x62aa08358c805, 0x46f440848e194, 0x447b771a8f52b, 0x377ba3269d31d, 0x3bf9baf55080 ]),
            y_minus_x: Fe([ 0x3c4277dbe5fde, 0x5a335afd44c92, 0xc1164099753e, 0x70487006fe423, 0x25e61cabed66f ]),
            xy2d: Fe([ 0x3e128cc586604, 0x5968b2e8fc7e2, 0x49a3d5bd61cf, 0x116505b1ef6e6, 0x566d78634586e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x54285c65a2fd0, 0x55e62ccf87420, 0x46bb961b19044, 0x1153405712039, 0x14fba5f34793b ]),
            y_minus_x: Fe([ 0x7a49f9cc10834, 0x2b513788a22c6, 0x5ff4b6ef2395b, 0x2ec8e5af607bf, 0x33975bca5ecc3 ]),
            xy2d: Fe([ 0x746166985f7d4, 0x9939000ae79a, 0x5844c7964f97a, 0x13617e1f95b3d, 0x14829cea83fc5 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x70b2f4e71ecb8, 0x728148efc643c, 0x753e03995b76, 0x5bf5fb2ab6767, 0x5fc3bc4535d7 ]),
            y_minus_x: Fe([ 0x37b8497dd95c2, 0x61549d6b4ffe8, 0x217a22db1d138, 0xb9cf062eb09e, 0x2fd9c71e5f758 ]),
            xy2d: Fe([ 0xb3ae52afdedd, 0x19da76619e497, 0x6fa0654d2558e, 0x78219d25e41d4, 0x373767475c651 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x95cb14246590, 0x2d82aa6ac68, 0x442f183bc4851, 0x6464f1c0a0644, 0x6bf5905730907 ]),
            y_minus_x: Fe([ 0x299fd40d1add9, 0x5f2de9a04e5f7, 0x7c0eebacc1c59, 0x4cca1b1f8290a, 0x1fbea56c3b18f ]),
            xy2d: Fe([ 0x778f1e1415b8a, 0x6f75874efc1f4, 0x28a694019027f, 0x52b37a96bdc4d, 0x2521cf67a635 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x46720772f5ee4, 0x632c0f359d622, 0x2b2092ba3e252, 0x662257c112680, 0x1753d9f7cd6 ]),
            y_minus_x: Fe([ 0x7ee0b0a9d5294, 0x381fbeb4cca27, 0x7841f3a3e639d, 0x676ea30c3445f, 0x3fa00a7e71382 ]),
            xy2d: Fe([ 0x1232d963ddb34, 0x35692e70b078d, 0x247ca14777a1f, 0x6db556be8fcd0, 0x12b5fe2fa048e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x37c26ad6f1e92, 0x46a0971227be5, 0x4722f0d2d9b4c, 0x3dc46204ee03a, 0x6f7e93c20796c ]),
            y_minus_x: Fe([ 0xfbc496fce34d, 0x575be6b7dae3e, 0x4a31585cee609, 0x37e9023930ff, 0x749b76f96fb12 ]),
            xy2d: Fe([ 0x2f604aea6ae05, 0x637dc939323eb, 0x3fdad9b048d47, 0xa8b0d4045af7, 0xfcec10f01e02 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2d29dc4244e45, 0x6927b1bc147be, 0x308534ac0839, 0x4853664033f41, 0x413779166feab ]),
            y_minus_x: Fe([ 0x558a649fe1e44, 0x44635aeefcc89, 0x1ff434887f2ba, 0xf981220e2d44, 0x4901aa7183c51 ]),
            xy2d: Fe([ 0x1b7548c1af8f0, 0x7848c53368116, 0x1b64e7383de9, 0x109fbb0587c8f, 0x41bb887b726d1 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x34c597c6691ae, 0x7a150b6990fc4, 0x52beb9d922274, 0x70eed7164861a, 0xa871e070c6a9 ]),
            y_minus_x: Fe([ 0x7d44744346be, 0x282b6a564a81d, 0x4ed80f875236b, 0x6fbbe1d450c50, 0x4eb728c12fcdb ]),
            xy2d: Fe([ 0x1b5994bbc8989, 0x74b7ba84c0660, 0x75678f1cdaeb8, 0x23206b0d6f10c, 0x3ee7300f2685d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x27947841e7518, 0x32c7388dae87f, 0x414add3971be9, 0x1850832f0ef1, 0x7d47c6a2cfb89 ]),
            y_minus_x: Fe([ 0x255e49e7dd6b7, 0x38c2163d59eba, 0x3861f2a005845, 0x2e11e4ccbaec9, 0x1381576297912 ]),
            xy2d: Fe([ 0x2d0148ef0d6e0, 0x3522a8de787fb, 0x2ee055e74f9d2, 0x64038f6310813, 0x148cf58d34c9e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x72f7d9ae4756d, 0x7711e690ffc4a, 0x582a2355b0d16, 0xdccfe885b6b4, 0x278febad4eaea ]),
            y_minus_x: Fe([ 0x492f67934f027, 0x7ded0815528d4, 0x58461511a6612, 0x5ea2e50de1544, 0x3ff2fa1ebd5db ]),
            xy2d: Fe([ 0x2681f8c933966, 0x3840521931635, 0x674f14a308652, 0x3bd9c88a94890, 0x4104dd02fe9c6 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x14e06db096ab8, 0x1219c89e6b024, 0x278abd486a2db, 0x240b292609520, 0x165b5a48efca ]),
            y_minus_x: Fe([ 0x2bf5e1124422a, 0x673146756ae56, 0x14ad99a87e830, 0x1eaca65b080fd, 0x2c863b00afaf5 ]),
            xy2d: Fe([ 0xa474a0846a76, 0x99a5ef981e32, 0x2a8ae3c4bbfe6, 0x45c34af14832c, 0x591b67d9bffec ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1b3719f18b55d, 0x754318c83d337, 0x27c17b7919797, 0x145b084089b61, 0x489b4f8670301 ]),
            y_minus_x: Fe([ 0x70d1c80b49bfa, 0x3d57e7d914625, 0x3c0722165e545, 0x5e5b93819e04f, 0x3de02ec7ca8f7 ]),
            xy2d: Fe([ 0x2102d3aeb92ef, 0x68c22d50c3a46, 0x42ea89385894e, 0x75f9ebf55f38c, 0x49f5fbba496cb ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5628c1e9c572e, 0x598b108e822ab, 0x55d8fae29361a, 0xadc8d1a97b28, 0x6a1a6c288675 ]),
            y_minus_x: Fe([ 0x49a108a5bcfd4, 0x6178c8e7d6612, 0x1f03473710375, 0x73a49614a6098, 0x5604a86dcbfa6 ]),
            xy2d: Fe([ 0xd1d47c1764b6, 0x1c08316a2e51, 0x2b3db45c95045, 0x1634f818d300c, 0x20989e89fe274 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4278b85eaec2e, 0xef59657be2ce, 0x72fd169588770, 0x2e9b205260b30, 0x730b9950f7059 ]),
            y_minus_x: Fe([ 0x777fd3a2dcc7f, 0x594a9fb124932, 0x1f8e80ca15f0, 0x714d13cec3269, 0x403ed1d0ca67 ]),
            xy2d: Fe([ 0x32d35874ec552, 0x1f3048df1b929, 0x300d73b179b23, 0x6e67be5a37d0b, 0x5bd7454308303 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4932115e7792a, 0x457b9bbb930b8, 0x68f5d8b193226, 0x4164e8f1ed456, 0x5bb7db123067f ]),
            y_minus_x: Fe([ 0x2d19528b24cc2, 0x4ac66b8302ff3, 0x701c8d9fdad51, 0x6c1b35c5b3727, 0x133a78007380a ]),
            xy2d: Fe([ 0x1f467c6ca62be, 0x2c4232a5dc12c, 0x7551dc013b087, 0x690c11b03bcd, 0x740dca6d58f0e ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x28c570478433c, 0x1d8502873a463, 0x7641e7eded49c, 0x1ecedd54cf571, 0x2c03f5256c2b0 ]),
            y_minus_x: Fe([ 0xee0752cfce4e, 0x660dd8116fbe9, 0x55167130fffeb, 0x1c682b885955c, 0x161d25fa963ea ]),
            xy2d: Fe([ 0x718757b53a47d, 0x619e18b0f2f21, 0x5fbdfe4c1ec04, 0x5d798c81ebb92, 0x699468bdbd96b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x53de66aa91948, 0x45f81a599b1b, 0x3f7a8bd214193, 0x71d4da412331a, 0x293e1c4e6c4a2 ]),
            y_minus_x: Fe([ 0x72f46f4dafecf, 0x2948ffadef7a3, 0x11ecdfdf3bc04, 0x3c2e98ffeed25, 0x525219a473905 ]),
            xy2d: Fe([ 0x6134b925112e1, 0x6bb942bb406ed, 0x70c445c0dde2, 0x411d822c4d7a3, 0x5b605c447f032 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1fec6f0e7f04c, 0x3cebc692c477d, 0x77986a19a95e, 0x6eaaaa1778b0f, 0x2f12fef4cc5ab ]),
            y_minus_x: Fe([ 0x5805920c47c89, 0x1924771f9972c, 0x38bbddf9fc040, 0x1f7000092b281, 0x24a76dcea8aeb ]),
            xy2d: Fe([ 0x522b2dfc0c740, 0x7e8193480e148, 0x33fd9a04341b9, 0x3c863678a20bc, 0x5e607b2518a43 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4431ca596cf14, 0x15da7c801405, 0x3c9b6f8f10b5, 0x346922934017, 0x201f33139e457 ]),
            y_minus_x: Fe([ 0x31d8f6cdf1818, 0x1f86c4b144b16, 0x39875b8d73e9d, 0x2fbf0d9ffa7b3, 0x5067acab6ccdd ]),
            xy2d: Fe([ 0x27f6b08039d51, 0x4802f8000dfaa, 0x9692a062c525, 0x1baea91075817, 0x397cba8862460 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5c3fbc81379e7, 0x41bbc255e2f02, 0x6a3f756998650, 0x1297fd4e07c42, 0x771b4022c1e1c ]),
            y_minus_x: Fe([ 0x13093f05959b2, 0x1bd352f2ec618, 0x75789b88ea86, 0x61d1117ea48b9, 0x2339d320766e6 ]),
            xy2d: Fe([ 0x5d986513a2fa7, 0x63f3a99e11b0f, 0x28a0ecfd6b26d, 0x53b6835e18d8f, 0x331a189219971 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x12f3a9d7572af, 0x10d00e953c4ca, 0x603df116f2f8a, 0x33dc276e0e088, 0x1ac9619ff649a ]),
            y_minus_x: Fe([ 0x66f45fb4f80c6, 0x3cc38eeb9fea2, 0x107647270db1f, 0x710f1ea740dc8, 0x31167c6b83bdf ]),
            xy2d: Fe([ 0x33842524b1068, 0x77dd39d30fe45, 0x189432141a0d0, 0x88fe4eb8c225, 0x612436341f08b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x349e31a2d2638, 0x137a7fa6b16c, 0x681ae92777edc, 0x222bfc5f8dc51, 0x1522aa3178d90 ]),
            y_minus_x: Fe([ 0x541db874e898d, 0x62d80fb841b33, 0x3e6ef027fa97, 0x7a03c9e9633e8, 0x46ebe2309e5ef ]),
            xy2d: Fe([ 0x2f5369614938, 0x356e5ada20587, 0x11bc89f6bf902, 0x36746419c8db, 0x45fe70f505243 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x24920c8951491, 0x107ec61944c5e, 0x72752e017c01f, 0x122b7dda2e97a, 0x16619f6db57a2 ]),
            y_minus_x: Fe([ 0x75a6960c0b8c, 0x6dde1c5e41b49, 0x42e3f516da341, 0x16a03fda8e79e, 0x428d1623a0e39 ]),
            xy2d: Fe([ 0x74a4401a308fd, 0x6ed4b9558109, 0x746f1f6a08867, 0x4636f5c6f2321, 0x1d81592d60bd3 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x5b69f7b85c5e8, 0x17a2d175650ec, 0x4cc3e6dbfc19e, 0x73e1d3873be0e, 0x3a5f6d51b0af8 ]),
            y_minus_x: Fe([ 0x68756a60dac5f, 0x55d757b8aec26, 0x3383df45f80bd, 0x6783f8c9f96a6, 0x20234a7789ecd ]),
            xy2d: Fe([ 0x20db67178b252, 0x73aa3da2c0eda, 0x79045c01c70d3, 0x1b37b15251059, 0x7cd682353cffe ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5cd6068acf4f3, 0x3079afc7a74cc, 0x58097650b64b4, 0x47fabac9c4e99, 0x3ef0253b2b2cd ]),
            y_minus_x: Fe([ 0x1a45bd887fab6, 0x65748076dc17c, 0x5b98000aa11a8, 0x4a1ecc9080974, 0x2838c8863bdc0 ]),
            xy2d: Fe([ 0x3b0cf4a465030, 0x22b8aef57a2d, 0x2ad0677e925ad, 0x4094167d7457a, 0x21dcb8a606a82 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x500fabe7731ba, 0x7cc53c3113351, 0x7cf65fe080d81, 0x3c5d966011ba1, 0x5d840dbf6c6f6 ]),
            y_minus_x: Fe([ 0x4468c9d9fc8, 0x5da8554796b8c, 0x3b8be70950025, 0x6d5892da6a609, 0xbc3d08194a31 ]),
            xy2d: Fe([ 0x6380d309fe18b, 0x4d73c2cb8ee0d, 0x6b882adbac0b6, 0x36eabdddd4cbe, 0x3a4276232ac19 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xc172db447ecb, 0x3f8c505b7a77f, 0x6a857f97f3f10, 0x4fcc0567fe03a, 0x770c9e824e1a ]),
            y_minus_x: Fe([ 0x2432c8a7084fa, 0x47bf73ca8a968, 0x1639176262867, 0x5e8df4f8010ce, 0x1ff177cea16de ]),
            xy2d: Fe([ 0x1d99a45b5b5fd, 0x523674f2499ec, 0xf8fa26182613, 0x58f7398048c98, 0x39f264fd41500 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x34aabfe097be1, 0x43bfc03253a33, 0x29bc7fe91b7f3, 0xa761e4844a16, 0x65c621272c35f ]),
            y_minus_x: Fe([ 0x53417dbe7e29c, 0x54573827394f5, 0x565eea6f650dd, 0x42050748dc749, 0x1712d73468889 ]),
            xy2d: Fe([ 0x389f8ce3193dd, 0x2d424b8177ce5, 0x73fa0d3440cd, 0x139020cd49e97, 0x22f9800ab19ce ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x29fdd9a6efdac, 0x7c694a9282840, 0x6f7cdeee44b3a, 0x55a3207b25cc3, 0x4171a4d38598c ]),
            y_minus_x: Fe([ 0x2368a3e9ef8cb, 0x454aa08e2ac0b, 0x490923f8fa700, 0x372aa9ea4582f, 0x13f416cd64762 ]),
            xy2d: Fe([ 0x758aa99c94c8c, 0x5f6001700ff44, 0x7694e488c01bd, 0xd5fde948eed6, 0x508214fa574bd ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x215bb53d003d6, 0x1179e792ca8c3, 0x1a0e96ac840a2, 0x22393e2bb3ab6, 0x3a7758a4c86cb ]),
            y_minus_x: Fe([ 0x269153ed6fe4b, 0x72a23aef89840, 0x52be5299699c, 0x3a5e5ef132316, 0x22f960ec6faba ]),
            xy2d: Fe([ 0x111f693ae5076, 0x3e3bfaa94ca90, 0x445799476b887, 0x24a0912464879, 0x5d9fd15f8de7f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x44d2aeed7521e, 0x50865d2c2a7e4, 0x2705b5238ea40, 0x46c70b25d3b97, 0x3bc187fa47eb9 ]),
            y_minus_x: Fe([ 0x408d36d63727f, 0x5faf8f6a66062, 0x2bb892da8de6b, 0x769d4f0c7e2e6, 0x332f35914f8fb ]),
            xy2d: Fe([ 0x70115ea86c20c, 0x16d88da24ada8, 0x1980622662adf, 0x501ebbc195a9d, 0x450d81ce906fb ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x4d8961cae743f, 0x6bdc38c7dba0e, 0x7d3b4a7e1b463, 0x844bdee2adf3, 0x4cbad279663ab ]),
            y_minus_x: Fe([ 0x3b6a1a6205275, 0x2e82791d06dcf, 0x23d72caa93c87, 0x5f0b7ab68aaf4, 0x2de25d4ba6345 ]),
            xy2d: Fe([ 0x19024a0d71fcd, 0x15f65115f101a, 0x4e99067149708, 0x119d8d1cba5af, 0x7d7fbcefe2007 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x45dc5f3c29094, 0x3455220b579af, 0x70c1631e068a, 0x26bc0630e9b21, 0x4f9cd196dcd8d ]),
            y_minus_x: Fe([ 0x71e6a266b2801, 0x9aae73e2df5d, 0x40dd8b219b1a3, 0x546fb4517de0d, 0x5975435e87b75 ]),
            xy2d: Fe([ 0x297d86a7b3768, 0x4835a2f4c6332, 0x70305f434160, 0x183dd014e56ae, 0x7ccdd084387a0 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x484186760cc93, 0x7435665533361, 0x2f686336b801, 0x5225446f64331, 0x3593ca848190c ]),
            y_minus_x: Fe([ 0x6422c6d260417, 0x212904817bb94, 0x5a319deb854f5, 0x7a9d4e060da7d, 0x428bd0ed61d0c ]),
            xy2d: Fe([ 0x3189a5e849aa7, 0x6acbb1f59b242, 0x7f6ef4753630c, 0x1f346292a2da9, 0x27398308da2d6 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x10e4c0a702453, 0x4daafa37bd734, 0x49f6bdc3e8961, 0x1feffdcecdae6, 0x572c2945492c3 ]),
            y_minus_x: Fe([ 0x38d28435ed413, 0x4064f19992858, 0x7680fbef543cd, 0x1aadd83d58d3c, 0x269597aebe8c3 ]),
            xy2d: Fe([ 0x7c745d6cd30be, 0x27c7755df78ef, 0x1776833937fa3, 0x5405116441855, 0x7f985498c05bc ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x615520fbf6363, 0xb9e9bf74da6a, 0x4fe8308201169, 0x173f76127de43, 0x30f2653cd69b1 ]),
            y_minus_x: Fe([ 0x1ce889f0be117, 0x36f6a94510709, 0x7f248720016b4, 0x1821ed1e1cf91, 0x76c2ec470a31f ]),
            xy2d: Fe([ 0xc938aac10c85, 0x41b64ed797141, 0x1beb1c1185e6d, 0x1ed5490600f07, 0x2f1273f159647 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x8bd755a70bc0, 0x49e3a885ce609, 0x16585881b5ad6, 0x3c27568d34f5e, 0x38ac1997edc5f ]),
            y_minus_x: Fe([ 0x1fc7c8ae01e11, 0x2094d5573e8e7, 0x5ca3cbbf549d2, 0x4f920ecc54143, 0x5d9e572ad85b6 ]),
            xy2d: Fe([ 0x6b517a751b13b, 0xcfd370b180cc, 0x5377925d1f41a, 0x34e56566008a2, 0x22dfcd9cbfe9e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x459b4103be0a1, 0x59a4b3f2d2add, 0x7d734c8bb8eeb, 0x2393cbe594a09, 0xfe9877824cde ]),
            y_minus_x: Fe([ 0x3d2e0c30d0cd9, 0x3f597686671bb, 0xaa587eb63999, 0xe3c7b592c619, 0x6b2916c05448c ]),
            xy2d: Fe([ 0x334d10aba913b, 0x45cdb581cfdb, 0x5e3e0553a8f36, 0x50bb3041effb2, 0x4c303f307ff00 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x403580dd94500, 0x48df77d92653f, 0x38a9fe3b349ea, 0xea89850aafe1, 0x416b151ab706a ]),
            y_minus_x: Fe([ 0x23bd617b28c85, 0x6e72ee77d5a61, 0x1a972ff174dde, 0x3e2636373c60f, 0xd61b8f78b2ab ]),
            xy2d: Fe([ 0xd7efe9c136b0, 0x1ab1c89640ad5, 0x55f82aef41f97, 0x46957f317ed0d, 0x191a2af74277e ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x62b434f460efb, 0x294c6c0fad3fc, 0x68368937b4c0f, 0x5c9f82910875b, 0x237e7dbe00545 ]),
            y_minus_x: Fe([ 0x6f74bc53c1431, 0x1c40e5dbbd9c2, 0x6c8fb9cae5c97, 0x4845c5ce1b7da, 0x7e2e0e450b5cc ]),
            xy2d: Fe([ 0x575ed6701b430, 0x4d3e17fa20026, 0x791fc888c4253, 0x2f1ba99078ac1, 0x71afa699b1115 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x23c1c473b50d6, 0x3e7671de21d48, 0x326fa5547a1e8, 0x50e4dc25fafd9, 0x731fbc78f89 ]),
            y_minus_x: Fe([ 0x66f9b3953b61d, 0x555f4283cccb9, 0x7dd67fb1960e7, 0x14707a1affed4, 0x21142e9c2b1c ]),
            xy2d: Fe([ 0xc71848f81880, 0x44bd9d8233c86, 0x6e8578efe5830, 0x4045b6d7041b5, 0x4c4d6f3347e15 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4ddfc988f1970, 0x4f6173ea365e1, 0x645daf9ae4588, 0x7d43763db623b, 0x38bf9500a88f9 ]),
            y_minus_x: Fe([ 0x7eccfc17d1fc9, 0x4ca280782831e, 0x7b8337db1d7d6, 0x5116def3895fb, 0x193fddaaa7e47 ]),
            xy2d: Fe([ 0x2c93c37e8876f, 0x3431a28c583fa, 0x49049da8bd879, 0x4b4a8407ac11c, 0x6a6fb99ebf0d4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x122b5b6e423c6, 0x21e50dff1ddd6, 0x73d76324e75c0, 0x588485495418e, 0x136fda9f42c5e ]),
            y_minus_x: Fe([ 0x6c1bb560855eb, 0x71f127e13ad48, 0x5c6b304905aec, 0x3756b8e889bc7, 0x75f76914a3189 ]),
            xy2d: Fe([ 0x4dfb1a305bdd1, 0x3b3ff05811f29, 0x6ed62283cd92e, 0x65d1543ec52e1, 0x22183510be8d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2710143307a7f, 0x3d88fb48bf3ab, 0x249eb4ec18f7a, 0x136115dff295f, 0x1387c441fd404 ]),
            y_minus_x: Fe([ 0x766385ead2d14, 0x194f8b06095e, 0x8478f6823b62, 0x6018689d37308, 0x6a071ce17b806 ]),
            xy2d: Fe([ 0x3c3d187978af8, 0x7afe1c88276ba, 0x51df281c8ad68, 0x64906bda4245d, 0x3171b26aaf1ed ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5b7d8b28a47d1, 0x2c2ee149e34c1, 0x776f5629afc53, 0x1f4ea50fc49a9, 0x6c514a6334424 ]),
            y_minus_x: Fe([ 0x7319097564ca8, 0x1844ebc233525, 0x21d4543fdeee1, 0x1ad27aaff1bd2, 0x221fd4873cf08 ]),
            xy2d: Fe([ 0x2204f3a156341, 0x537414065a464, 0x43c0c3bedcf83, 0x5557e706ea620, 0x48daa596fb924 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x61d5dc84c9793, 0x47de83040c29e, 0x189deb26507e7, 0x4d4e6fadc479a, 0x58c837fa0e8a7 ]),
            y_minus_x: Fe([ 0x28e665ca59cc7, 0x165c715940dd9, 0x785f3aa11c95, 0x57b98d7e38469, 0x676dd6fccad84 ]),
            xy2d: Fe([ 0x1688596fc9058, 0x66f6ad403619f, 0x4d759a87772ef, 0x7856e6173bea4, 0x1c4f73f2c6a57 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6706efc7c3484, 0x6987839ec366d, 0x731f95cf7f26, 0x3ae758ebce4bc, 0x70459adb7daf6 ]),
            y_minus_x: Fe([ 0x24fbd305fa0bb, 0x40a98cc75a1cf, 0x78ce1220a7533, 0x6217a10e1c197, 0x795ac80d1bf64 ]),
            xy2d: Fe([ 0x1db4991b42bb3, 0x469605b994372, 0x631e3715c9a58, 0x7e9cfefcf728f, 0x5fe162848ce21 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x1852d5d7cb208, 0x60d0fbe5ce50f, 0x5a1e246e37b75, 0x51aee05ffd590, 0x2b44c043677da ]),
            y_minus_x: Fe([ 0x1214fe194961a, 0xe1ae39a9e9cb, 0x543c8b526f9f7, 0x119498067e91d, 0x4789d446fc917 ]),
            xy2d: Fe([ 0x487ab074eb78e, 0x1d33b5e8ce343, 0x13e419feb1b46, 0x2721f565de6a4, 0x60c52eef2bb9a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x3c5c27cae6d11, 0x36a9491956e05, 0x124bac9131da6, 0x3b6f7de202b5d, 0x70d77248d9b66 ]),
            y_minus_x: Fe([ 0x589bc3bfd8bf1, 0x6f93e6aa3416b, 0x4c0a3d6c1ae48, 0x55587260b586a, 0x10bc9c312ccfc ]),
            xy2d: Fe([ 0x2e84b3ec2a05b, 0x69da2f03c1551, 0x23a174661a67b, 0x209bca289f238, 0x63755bd3a976f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7101897f1acb7, 0x3d82cb77b07b8, 0x684083d7769f5, 0x52b28472dce07, 0x2763751737c52 ]),
            y_minus_x: Fe([ 0x7a03e2ad10853, 0x213dcc6ad36ab, 0x1a6e240d5bdd6, 0x7c24ffcf8fedf, 0xd8cc1c48bc16 ]),
            xy2d: Fe([ 0x402d36eb419a9, 0x7cef68c14a052, 0xf1255bc2d139, 0x373e7d431186a, 0x70c2dd8a7ad16 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4967db8ed7e13, 0x15aeed02f523a, 0x6149591d094bc, 0x672f204c17006, 0x32b8613816a53 ]),
            y_minus_x: Fe([ 0x194509f6fec0e, 0x528d8ca31acac, 0x7826d73b8b9fa, 0x24acb99e0f9b3, 0x2e0fac6363948 ]),
            xy2d: Fe([ 0x7f7bee448cd64, 0x4e10f10da0f3c, 0x3936cb9ab20e9, 0x7a0fc4fea6cd0, 0x4179215c735a4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x633b9286bcd34, 0x6cab3badb9c95, 0x74e387edfbdfa, 0x14313c58a0fd9, 0x31fa85662241c ]),
            y_minus_x: Fe([ 0x94e7d7dced2a, 0x68fa738e118e, 0x41b640a5fee2b, 0x6bb709df019d4, 0x700344a30cd99 ]),
            xy2d: Fe([ 0x26c422e3622f4, 0xf3066a05b5f0, 0x4e2448f0480a6, 0x244cde0dbf095, 0x24bb2312a9952 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xc2af5f85c6b, 0x609f4cf2883f, 0x6e86eb5a1ca13, 0x68b44a2efccd1, 0xd1d2af9ffeb5 ]),
            y_minus_x: Fe([ 0xed1732de67c3, 0x308c369291635, 0x33ef348f2d250, 0x4475ea1a1bb, 0xfee3e871e188 ]),
            xy2d: Fe([ 0x28aa132621edf, 0x42b244caf353b, 0x66b064cc2e08a, 0x6bb20020cbdd3, 0x16acd79718531 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1c6c57887b6ad, 0x5abf21fd7592b, 0x50bd41253867a, 0x3800b71273151, 0x164ed34b18161 ]),
            y_minus_x: Fe([ 0x772af2d9b1d3d, 0x6d486448b4e5b, 0x2ce58dd8d18a8, 0x1849f67503c8b, 0x123e0ef6b9302 ]),
            xy2d: Fe([ 0x6d94c192fe69a, 0x5475222a2690f, 0x693789d86b8b3, 0x1f5c3bdfb69dc, 0x78da0fc61073f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x780f1680c3a94, 0x2a35d3cfcd453, 0x5e5cdc7ddf8, 0x6ee888078ac24, 0x54aa4b316b38 ]),
            y_minus_x: Fe([ 0x15d28e52bc66a, 0x30e1e0351cb7e, 0x30a2f74b11f8c, 0x39d120cd7de03, 0x2d25deeb256b1 ]),
            xy2d: Fe([ 0x468d19267cb8, 0x38cdca9b5fbf9, 0x1bbb05c2ca1e2, 0x3b015758e9533, 0x134610a6ab7da ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x265e777d1f515, 0xf1f54c1e39a5, 0x2f01b95522646, 0x4fdd8db9dde6d, 0x654878cba97cc ]),
            y_minus_x: Fe([ 0x38ec78df6b0fe, 0x13caebea36a22, 0x5ebc6e54e5f6a, 0x32804903d0eb8, 0x2102fdba2b20d ]),
            xy2d: Fe([ 0x6e405055ce6a1, 0x5024a35a532d3, 0x1f69054daf29d, 0x15d1d0d7a8bd5, 0xad725db29ecb ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7bc0c9b056f85, 0x51cfebffaffd8, 0x44abbe94df549, 0x7ecbbd7e33121, 0x4f675f5302399 ]),
            y_minus_x: Fe([ 0x267b1834e2457, 0x6ae19c378bb88, 0x7457b5ed9d512, 0x3280d783d05fb, 0x4aefcffb71a03 ]),
            xy2d: Fe([ 0x536360415171e, 0x2313309077865, 0x251444334afbc, 0x2b0c3853756e8, 0xbccbb72a2a86 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x55e4c50fe1296, 0x5fdd13efc30d, 0x1c0c6c380e5ee, 0x3e11de3fb62a8, 0x6678fd69108f3 ]),
            y_minus_x: Fe([ 0x6962feab1a9c8, 0x6aca28fb9a30b, 0x56db7ca1b9f98, 0x39f58497018dd, 0x4024f0ab59d6b ]),
            xy2d: Fe([ 0x6fa31636863c2, 0x10ae5a67e42b0, 0x27abbf01fda31, 0x380a7b9e64fbc, 0x2d42e2108ead4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x17b0d0f537593, 0x16263c0c9842e, 0x4ab827e4539a4, 0x6370ddb43d73a, 0x420bf3a79b423 ]),
            y_minus_x: Fe([ 0x5131594dfd29b, 0x3a627e98d52fe, 0x1154041855661, 0x19175d09f8384, 0x676b2608b8d2d ]),
            xy2d: Fe([ 0xba651c5b2b47, 0x5862363701027, 0xc4d6c219c6db, 0xf03dff8658de, 0x745d2ffa9c0cf ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6df5721d34e6a, 0x4f32f767a0c06, 0x1d5abeac76e20, 0x41ce9e104e1e4, 0x6e15be54c1dc ]),
            y_minus_x: Fe([ 0x25a1e2bc9c8bd, 0x104c8f3b037ea, 0x405576fa96c98, 0x2e86a88e3876f, 0x1ae23ceb960cf ]),
            xy2d: Fe([ 0x25d871932994a, 0x6b9d63b560b6e, 0x2df2814c8d472, 0xfbbee20aa4ed, 0x58ded861278ec ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x35ba8b6c2c9a8, 0x1dea58b3185bf, 0x4b455cd23bbbe, 0x5ec19c04883f8, 0x8ba696b531d5 ]),
            y_minus_x: Fe([ 0x73793f266c55c, 0xb988a9c93b02, 0x9b0ea32325db, 0x37cae71c17c5e, 0x2ff39de85485f ]),
            xy2d: Fe([ 0x53eeec3efc57a, 0x2fa9fe9022efd, 0x699c72c138154, 0x72a751ebd1ff8, 0x120633b4947cf ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x531474912100a, 0x5afcdf7c0d057, 0x7a9e71b788ded, 0x5ef708f3b0c88, 0x7433be3cb393 ]),
            y_minus_x: Fe([ 0x4987891610042, 0x79d9d7f5d0172, 0x3c293013b9ec4, 0xc2b85f39caca, 0x35d30a99b4d59 ]),
            xy2d: Fe([ 0x144c05ce997f4, 0x4960b8a347fef, 0x1da11f15d74f7, 0x54fac19c0fead, 0x2d873ede7af6d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x202e14e5df981, 0x2ea02bc3eb54c, 0x38875b2883564, 0x1298c513ae9dd, 0x543618a01600 ]),
            y_minus_x: Fe([ 0x2316443373409, 0x5de95503b22af, 0x699201beae2df, 0x3db5849ff737a, 0x2e773654707fa ]),
            xy2d: Fe([ 0x2bdf4974c23c1, 0x4b3b9c8d261bd, 0x26ae8b2a9bc28, 0x3068210165c51, 0x4b1443362d079 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x454e91c529ccb, 0x24c98c6bf72cf, 0x486594c3d89a, 0x7ae13a3d7fa3c, 0x17038418eaf66 ]),
            y_minus_x: Fe([ 0x4b7c7b66e1f7a, 0x4bea185efd998, 0x4fabc711055f8, 0x1fb9f7836fe38, 0x582f446752da6 ]),
            xy2d: Fe([ 0x17bd320324ce4, 0x51489117898c6, 0x1684d92a0410b, 0x6e4d90f78c5a7, 0xc2a1c4bcda28 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4814869bd6945, 0x7b7c391a45db8, 0x57316ac35b641, 0x641e31de9096a, 0x5a6a9b30a314d ]),
            y_minus_x: Fe([ 0x5c7d06f1f0447, 0x7db70f80b3a49, 0x6cb4a3ec89a78, 0x43be8ad81397d, 0x7c558bd1c6f64 ]),
            xy2d: Fe([ 0x41524d396463d, 0x1586b449e1a1d, 0x2f17e904aed8a, 0x7e1d2861d3c8e, 0x404a5ca0afba ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x49e1b2a416fd1, 0x51c6a0b316c57, 0x575a59ed71bdc, 0x74c021a1fec1e, 0x39527516e7f8e ]),
            y_minus_x: Fe([ 0x740070aa743d6, 0x16b64cbdd1183, 0x23f4b7b32eb43, 0x319aba58235b3, 0x46395bfdcadd9 ]),
            xy2d: Fe([ 0x7db2d1a5d9a9c, 0x79a200b85422f, 0x355bfaa71dd16, 0xb77ea5f78aa, 0x76579a29e822d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4b51352b434f2, 0x1327bd01c2667, 0x434d73b60c8a1, 0x3e0daa89443ba, 0x2c514bb2a277 ]),
            y_minus_x: Fe([ 0x68e7e49c02a17, 0x45795346fe8b6, 0x89306c8f3546, 0x6d89f6b2f88f6, 0x43a384dc9e05b ]),
            xy2d: Fe([ 0x3d5da8bf1b645, 0x7ded6a96a6d09, 0x6c3494fee2f4d, 0x2c989c8b6bd4, 0x1160920961548 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5616369b4dcd, 0x4ecab86ac6f47, 0x3c60085d700b2, 0x213ee10dfcea, 0x2f637d7491e6e ]),
            y_minus_x: Fe([ 0x5166929dacfaa, 0x190826b31f689, 0x4f55567694a7d, 0x705f4f7b1e522, 0x351e125bc5698 ]),
            xy2d: Fe([ 0x49b461af67bbe, 0x75915712c3a96, 0x69a67ef580c0d, 0x54d38ef70cffc, 0x7f182d06e7ce2 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x54b728e217522, 0x69a90971b0128, 0x51a40f2a963a3, 0x10be9ac12a6bf, 0x44acc043241c5 ]),
            y_minus_x: Fe([ 0x48e64ab0168ec, 0x2a2bdb8a86f4f, 0x7343b6b2d6929, 0x1d804aa8ce9a3, 0x67d4ac8c343e9 ]),
            xy2d: Fe([ 0x56bbb4f7a5777, 0x29230627c238f, 0x5ad1a122cd7fb, 0xdea56e50e364, 0x556d1c8312ad7 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6756b11be821, 0x462147e7bb03e, 0x26519743ebfe0, 0x782fc59682ab5, 0x97abe38cc8c7 ]),
            y_minus_x: Fe([ 0x740e30c8d3982, 0x7c2b47f4682fd, 0x5cd91b8c7dc1c, 0x77fa790f9e583, 0x746c6c6d1d824 ]),
            xy2d: Fe([ 0x1c9877ea52da4, 0x2b37b83a86189, 0x733af49310da5, 0x25e81161c04fb, 0x577e14a34bee8 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6cebebd4dd72b, 0x340c1e442329f, 0x32347ffd1a93f, 0x14a89252cbbe0, 0x705304b8fb009 ]),
            y_minus_x: Fe([ 0x268ac61a73b0a, 0x206f234bebe1c, 0x5b403a7cbebe8, 0x7a160f09f4135, 0x60fa7ee96fd78 ]),
            xy2d: Fe([ 0x51d354d296ec6, 0x7cbf5a63b16c7, 0x2f50bb3cf0c14, 0x1feb385cac65a, 0x21398e0ca1635 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0xaaf9b4b75601, 0x26b91b5ae44f3, 0x6de808d7ab1c8, 0x6a769675530b0, 0x1bbfb284e98f7 ]),
            y_minus_x: Fe([ 0x5058a382b33f3, 0x175a91816913e, 0x4f6cdb96b8ae8, 0x17347c9da81d2, 0x5aa3ed9d95a23 ]),
            xy2d: Fe([ 0x777e9c7d96561, 0x28e58f006ccac, 0x541bbbb2cac49, 0x3e63282994cec, 0x4a07e14e5e895 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x358cdc477a49b, 0x3cc88fe02e481, 0x721aab7f4e36b, 0x408cc9469953, 0x50af7aed84afa ]),
            y_minus_x: Fe([ 0x412cb980df999, 0x5e78dd8ee29dc, 0x171dff68c575d, 0x2015dd2f6ef49, 0x3f0bac391d313 ]),
            xy2d: Fe([ 0x7de0115f65be5, 0x4242c21364dc9, 0x6b75b64a66098, 0x33c0102c085, 0x1921a316baebd ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2ad9ad9f3c18b, 0x5ec1638339aeb, 0x5703b6559a83b, 0x3fa9f4d05d612, 0x7b049deca062c ]),
            y_minus_x: Fe([ 0x22f7edfb870fc, 0x569eed677b128, 0x30937dcb0a5af, 0x758039c78ea1b, 0x6458df41e273a ]),
            xy2d: Fe([ 0x3e37a35444483, 0x661fdb7d27b99, 0x317761dd621e4, 0x7323c30026189, 0x6093dccbc2950 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6eebe6084034b, 0x6cf01f70a8d7b, 0xb41a54c6670a, 0x6c84b99bb55db, 0x6e3180c98b647 ]),
            y_minus_x: Fe([ 0x39a8585e0706d, 0x3167ce72663fe, 0x63d14ecdb4297, 0x4be21dcf970b8, 0x57d1ea084827a ]),
            xy2d: Fe([ 0x2b6e7a128b071, 0x5b27511755dcf, 0x8584c2930565, 0x68c7bda6f4159, 0x363e999ddd97b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x48dce24baec6, 0x2b75795ec05e3, 0x3bfa4c5da6dc9, 0x1aac8659e371e, 0x231f979bc6f9b ]),
            y_minus_x: Fe([ 0x43c135ee1fc4, 0x2a11c9919f2d5, 0x6334cc25dbacd, 0x295da17b400da, 0x48ee9b78693a0 ]),
            xy2d: Fe([ 0x1de4bcc2af3c6, 0x61fc411a3eb86, 0x53ed19ac12ec0, 0x209dbc6b804e0, 0x79bfa9b08792 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1ed80a2d54245, 0x70efec72a5e79, 0x42151d42a822d, 0x1b5ebb6d631e8, 0x1ef4fb1594706 ]),
            y_minus_x: Fe([ 0x3a51da300df4, 0x467b52b561c72, 0x4d5920210e590, 0xca769e789685, 0x38c77f684817 ]),
            xy2d: Fe([ 0x65ee65b167bec, 0x52da19b850a9, 0x408665656429, 0x7ab39596f9a4c, 0x575ee92a4a0bf ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6bc450aa4d801, 0x4f4a6773b0ba8, 0x6241b0b0ebc48, 0x40d9c4f1d9315, 0x200a1e7e382f5 ]),
            y_minus_x: Fe([ 0x80908a182fcf, 0x532913b7ba98, 0x3dccf78c385c3, 0x68002dd5eaba9, 0x43d4e7112cd3f ]),
            xy2d: Fe([ 0x5b967eaf93ac5, 0x360acca580a31, 0x1c65fd5c6f262, 0x71c7f15c2ecab, 0x50eca52651e4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4397660e668ea, 0x7c2a75692f2f5, 0x3b29e7e6c66ef, 0x72ba658bcda9a, 0x6151c09fa131a ]),
            y_minus_x: Fe([ 0x31ade453f0c9c, 0x3dfee07737868, 0x611ecf7a7d411, 0x2637e6cbd64f6, 0x4b0ee6c21c58f ]),
            xy2d: Fe([ 0x55c0dfdf05d96, 0x405569dcf475e, 0x5c5c277498bb, 0x18588d95dc389, 0x1fef24fa800f0 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x2aff530976b86, 0xd85a48c0845a, 0x796eb963642e0, 0x60bee50c4b626, 0x28005fe6c8340 ]),
            y_minus_x: Fe([ 0x653fb1aa73196, 0x607faec8306fa, 0x4e85ec83e5254, 0x9f56900584fd, 0x544d49292fc86 ]),
            xy2d: Fe([ 0x7ba9f34528688, 0x284a20fb42d5d, 0x3652cd9706ffe, 0x6fd7baddde6b3, 0x72e472930f316 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x3f635d32a7627, 0xcbecacde00fe, 0x3411141eaa936, 0x21c1e42f3cb94, 0x1fee7f000fe06 ]),
            y_minus_x: Fe([ 0x5208c9781084f, 0x16468a1dc24d2, 0x7bf780ac540a8, 0x1a67eced75301, 0x5a9d2e8c2733a ]),
            xy2d: Fe([ 0x305da03dbf7e5, 0x1228699b7aeca, 0x12a23b2936bc9, 0x2a1bda56ae6e9, 0xf94051ee040 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x793bb07af9753, 0x1e7b6ecd4fafd, 0x2c7b1560fb43, 0x2296734cc5fb7, 0x47b7ffd25dd40 ]),
            y_minus_x: Fe([ 0x56b23c3d330b2, 0x37608e360d1a6, 0x10ae0f3c8722e, 0x86d9b618b637, 0x7d79c7e8beab ]),
            xy2d: Fe([ 0x3fb9cbc08dd12, 0x75c3dd85370ff, 0x47f06fe2819ac, 0x5db06ab9215ed, 0x1c3520a35ea64 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6f40216bc059, 0x3a2579b0fd9b5, 0x71c26407eec8c, 0x72ada4ab54f0b, 0x38750c3b66d12 ]),
            y_minus_x: Fe([ 0x253a6bccba34a, 0x427070433701a, 0x20b8e58f9870e, 0x337c861db00cc, 0x1c3d05775d0ee ]),
            xy2d: Fe([ 0x6f1409422e51a, 0x7856bbece2d25, 0x13380a72f031c, 0x43e1080a7f3ba, 0x621e2c7d3304 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x61796b0dbf0f3, 0x73c2f9c32d6f5, 0x6aa8ed1537ebe, 0x74e92c91838f4, 0x5d8e589ca1002 ]),
            y_minus_x: Fe([ 0x60cc8259838d, 0x38d3f35b95f3, 0x56078c243a923, 0x2de3293241bb2, 0x7d6097bd3a ]),
            xy2d: Fe([ 0x71d950842a94b, 0x46b11e5c7d817, 0x5478bbecb4f0d, 0x7c3054b0a1c5d, 0x1583d7783c1cb ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x34704cc9d28c7, 0x3dee598b1f200, 0x16e1c98746d9e, 0x4050b7095afdf, 0x4958064e83c55 ]),
            y_minus_x: Fe([ 0x6a2ef5da27ae1, 0x28aace02e9d9d, 0x2459e965f0e8, 0x7b864d3150933, 0x252a5f2e81ed8 ]),
            xy2d: Fe([ 0x94265066e80d, 0xa60f918d61a5, 0x444bf7f30fde, 0x1c40da9ed3c06, 0x79c170bd843b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6cd50c0d5d056, 0x5b7606ae779ba, 0x70fbd226bdda1, 0x5661e53391ff9, 0x6768c0d7317b8 ]),
            y_minus_x: Fe([ 0x6ece464fa6fff, 0x3cc40bca460a0, 0x6e3a90afb8d0c, 0x5801abca11228, 0x6dec05e34ac9f ]),
            xy2d: Fe([ 0x625e5f155c1b3, 0x4f32f6f723296, 0x5ac980105efce, 0x17a61165eee36, 0x51445e14ddcd5 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x147ab2bbea455, 0x1f240f2253126, 0xc3de9e314e89, 0x21ea5a4fca45f, 0x12e990086e4fd ]),
            y_minus_x: Fe([ 0x2b4b3b144951, 0x5688977966aea, 0x18e176e399ffd, 0x2e45c5eb4938b, 0x13186f31e3929 ]),
            xy2d: Fe([ 0x496b37fdfbb2e, 0x3c2439d5f3e21, 0x16e60fe7e6a4d, 0x4d7ef889b621d, 0x77b2e3f05d3e9 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x639c12ddb0a4, 0x6180490cd7ab3, 0x3f3918297467c, 0x74568be1781ac, 0x7a195152e095 ]),
            y_minus_x: Fe([ 0x7a9c59c2ec4de, 0x7e9f09e79652d, 0x6a3e422f22d86, 0x2ae8e3b836c8b, 0x63b795fc7ad32 ]),
            xy2d: Fe([ 0x68f02389e5fc8, 0x59f1bc877506, 0x504990e410cec, 0x9bd7d0feaee2, 0x3e8fe83d032f0 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4c8de8efd13c, 0x1c67c06e6210e, 0x183378f7f146a, 0x64352ceaed289, 0x22d60899a6258 ]),
            y_minus_x: Fe([ 0x315b90570a294, 0x60ce108a925f1, 0x6eff61253c909, 0x3ef0e2d70b0, 0x75ba3b797fac4 ]),
            xy2d: Fe([ 0x1dbc070cdd196, 0x16d8fb1534c47, 0x500498183fa2a, 0x72f59c423de75, 0x904d07b87779 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x22d6648f940b9, 0x197a5a1873e86, 0x207e4c41a54bc, 0x5360b3b4bd6d0, 0x6240aacebaf72 ]),
            y_minus_x: Fe([ 0x61fd4ddba919c, 0x7d8e991b55699, 0x61b31473cc76c, 0x7039631e631d6, 0x43e2143fbc1dd ]),
            xy2d: Fe([ 0x4749c5ba295a0, 0x37946fa4b5f06, 0x724c5ab5a51f1, 0x65633789dd3f3, 0x56bdaf238db40 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xd36cc19d3bb2, 0x6ec4470d72262, 0x6853d7018a9ae, 0x3aa3e4dc2c8eb, 0x3aa31507e1e5 ]),
            y_minus_x: Fe([ 0x2b9e3f53533eb, 0x2add727a806c5, 0x56955c8ce15a3, 0x18c4f070a290e, 0x1d24a86d83741 ]),
            xy2d: Fe([ 0x47648ffd4ce1f, 0x60a9591839e9d, 0x424d5f38117ab, 0x42cc46912c10e, 0x43b261dc9aeb4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x13d8b6c951364, 0x4c0017e8f632a, 0x53e559e53f9c4, 0x4b20146886eea, 0x2b4d5e242940 ]),
            y_minus_x: Fe([ 0x31e1988bb79bb, 0x7b82f46b3bcab, 0xf7a8ce827b41, 0x5e15816177130, 0x326055cf5b276 ]),
            xy2d: Fe([ 0x155cb28d18df2, 0xc30d9ca11694, 0x2090e27ab3119, 0x208624e7a49b6, 0x27a6c809ae5d3 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4270ac43d6954, 0x2ed4cd95659a5, 0x75c0db37528f9, 0x2ccbcfd2c9234, 0x221503603d8c2 ]),
            y_minus_x: Fe([ 0x6ebcd1f0db188, 0x74ceb4b7d1174, 0x7d56168df4f5c, 0xbf79176fd18a, 0x2cb67174ff60a ]),
            xy2d: Fe([ 0x6cdf9390be1d0, 0x8e519c7e2b3d, 0x253c3d2a50881, 0x21b41448e333d, 0x7b1df4b73890f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6221807f8f58c, 0x3fa92813a8be5, 0x6da98c38d5572, 0x1ed95554468f, 0x68698245d352e ]),
            y_minus_x: Fe([ 0x2f2e0b3b2a224, 0xc56aa22c1c92, 0x5fdec39f1b278, 0x4c90af5c7f106, 0x61fcef2658fc5 ]),
            xy2d: Fe([ 0x15d852a18187a, 0x270dbb59afb76, 0x7db120bcf92ab, 0xe7a25d714087, 0x46cf4c473daf0 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x46ea7f1498140, 0x70725690a8427, 0xa73ae9f079fb, 0x2dd924461c62b, 0x1065aae50d8cc ]),
            y_minus_x: Fe([ 0x525ed9ec4e5f9, 0x22d20660684c, 0x7972b70397b68, 0x7a03958d3f965, 0x29387bcd14eb5 ]),
            xy2d: Fe([ 0x44525df200d57, 0x2d7f94ce94385, 0x60d00c170ecb7, 0x38b0503f3d8f0, 0x69a198e64f1ce ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x14434dcc5caed, 0x2c7909f667c20, 0x61a839d1fb576, 0x4f23800cabb76, 0x25b2697bd267f ]),
            y_minus_x: Fe([ 0x2b2e0d91a78bc, 0x3990a12ccf20c, 0x141c2e11f2622, 0xdfcefaa53320, 0x7369e6a92493a ]),
            xy2d: Fe([ 0x73ffb13986864, 0x3282bb8f713ac, 0x49ced78f297ef, 0x6697027661def, 0x1420683db54e4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6bb6fc1cc5ad0, 0x532c8d591669d, 0x1af794da86c33, 0xe0e9d86d24d3, 0x31e83b4161d08 ]),
            y_minus_x: Fe([ 0xbd1e249dd197, 0xbcb1820568f, 0x2eab1718830d4, 0x396fd816997e6, 0x60b63bebf508a ]),
            xy2d: Fe([ 0xc7129e062b4f, 0x1e526415b12fd, 0x461a0fd27923d, 0x18badf670a5b7, 0x55cf1eb62d550 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6b5e37df58c52, 0x3bcf33986c60e, 0x44fb8835ceae7, 0x99dec18e71a4, 0x1a56fbaa62ba0 ]),
            y_minus_x: Fe([ 0x1101065c23d58, 0x5aa1290338b0f, 0x3157e9e2e7421, 0xea712017d489, 0x669a656457089 ]),
            xy2d: Fe([ 0x66b505c9dc9ec, 0x774ef86e35287, 0x4d1d944c0955e, 0x52e4c39d72b20, 0x13c4836799c58 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4fb6a5d8bd080, 0x58ae34908589b, 0x3954d977baf13, 0x413ea597441dc, 0x50bdc87dc8e5b ]),
            y_minus_x: Fe([ 0x25d465ab3e1b9, 0xf8fe27ec2847, 0x2d6e6dbf04f06, 0x3038cfc1b3276, 0x66f80c93a637b ]),
            xy2d: Fe([ 0x537836edfe111, 0x2be02357b2c0d, 0x6dcee58c8d4f8, 0x2d732581d6192, 0x1dd56444725fd ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7e60008bac89a, 0x23d5c387c1852, 0x79e5df1f533a8, 0x2e6f9f1c5f0cf, 0x3a3a450f63a30 ]),
            y_minus_x: Fe([ 0x47ff83362127d, 0x8e39af82b1f4, 0x488322ef27dab, 0x1973738a2a1a4, 0xe645912219f7 ]),
            xy2d: Fe([ 0x72f31d8394627, 0x7bd294a200f1, 0x665be00e274c6, 0x43de8f1b6368b, 0x318c8d9393a9a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x69e29ab1dd398, 0x30685b3c76bac, 0x565cf37f24859, 0x57b2ac28efef9, 0x509a41c325950 ]),
            y_minus_x: Fe([ 0x45d032afffe19, 0x12fe49b6cde4e, 0x21663bc327cf1, 0x18a5e4c69f1dd, 0x224c7c679a1d5 ]),
            xy2d: Fe([ 0x6edca6f925e9, 0x68c8363e677b8, 0x60cfa25e4fbcf, 0x1c4c17609404e, 0x5bff02328a11 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1a0dd0dc512e4, 0x10894bf5fcd10, 0x52949013f9c37, 0x1f50fba4735c7, 0x576277cdee01a ]),
            y_minus_x: Fe([ 0x2137023cae00b, 0x15a3599eb26c6, 0x687221512b3c, 0x253cb3a0824e9, 0x780b8cc3fa2a4 ]),
            xy2d: Fe([ 0x38abc234f305f, 0x7a280bbc103de, 0x398a836695dfe, 0x3d0af41528a1a, 0x5ff418726271b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x347e813b69540, 0x76864c21c3cbb, 0x1e049dbcd74a8, 0x5b4d60f93749c, 0x29d4db8ca0a0c ]),
            y_minus_x: Fe([ 0x6080c1789db9d, 0x4be7cef1ea731, 0x2f40d769d8080, 0x35f7d4c44a603, 0x106a03dc25a96 ]),
            xy2d: Fe([ 0x50aaf333353d0, 0x4b59a613cbb35, 0x223dfc0e19a76, 0x77d1e2bb2c564, 0x4ab38a51052cb ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x7d1ef5fddc09c, 0x7beeaebb9dad9, 0x58d30ba0acfb, 0x5cd92eab5ae90, 0x3041c6bb04ed2 ]),
            y_minus_x: Fe([ 0x42b256768d593, 0x2e88459427b4f, 0x2b3876630701, 0x34878d405eae5, 0x29cdd1adc088a ]),
            xy2d: Fe([ 0x2f2f9d956e148, 0x6b3e6ad65c1fe, 0x5b00972b79e5d, 0x53d8d234c5daf, 0x104bbd6814049 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x59a5fd67ff163, 0x3a998ead0352b, 0x83c95fa4af9a, 0x6fadbfc01266f, 0x204f2a20fb072 ]),
            y_minus_x: Fe([ 0xfd3168f1ed67, 0x1bb0de7784a3e, 0x34bcb78b20477, 0xa4a26e2e2182, 0x5be8cc57092a7 ]),
            xy2d: Fe([ 0x43b3d30ebb079, 0x357aca5c61902, 0x5b570c5d62455, 0x30fb29e1e18c7, 0x2570fb17c2791 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6a9550bb8245a, 0x511f20a1a2325, 0x29324d7239bee, 0x3343cc37516c4, 0x241c5f91de018 ]),
            y_minus_x: Fe([ 0x2367f2cb61575, 0x6c39ac04d87df, 0x6d4958bd7e5bd, 0x566f4638a1532, 0x3dcb65ea53030 ]),
            xy2d: Fe([ 0x172940de6caa, 0x6045b2e67451b, 0x56c07463efcb3, 0x728b6bfe6e91, 0x8420edd5fcdf ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xc34e04f410ce, 0x344edc0d0a06b, 0x6e45486d84d6d, 0x44e2ecb3863f5, 0x4d654f321db8 ]),
            y_minus_x: Fe([ 0x720ab8362fa4a, 0x29c4347cdd9bf, 0xe798ad5f8463, 0x4fef18bcb0bfe, 0xd9a53efbc176 ]),
            xy2d: Fe([ 0x5c116ddbdb5d5, 0x6d1b4bba5abcf, 0x4d28a48a5537a, 0x56b8e5b040b99, 0x4a7a4f2618991 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x3b291af372a4b, 0x60e3028fe4498, 0x2267bca4f6a09, 0x719eec242b243, 0x4a96314223e0e ]),
            y_minus_x: Fe([ 0x718025fb15f95, 0x68d6b8371fe94, 0x3804448f7d97c, 0x42466fe784280, 0x11b50c4cddd31 ]),
            xy2d: Fe([ 0x274408a4ffd6, 0x7d382aedb34dd, 0x40acfc9ce385d, 0x628bb99a45b1e, 0x4f4bce4dce6bc ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2616ec49d0b6f, 0x1f95d8462e61c, 0x1ad3e9b9159c6, 0x79ba475a04df9, 0x3042cee561595 ]),
            y_minus_x: Fe([ 0x7ce5ae2242584, 0x2d25eb153d4e3, 0x3a8f3d09ba9c9, 0xf3690d04eb8e, 0x73fcdd14b71c0 ]),
            xy2d: Fe([ 0x67079449bac41, 0x5b79c4621484f, 0x61069f2156b8d, 0xeb26573b10af, 0x389e740c9a9ce ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x578f6570eac28, 0x644f2339c3937, 0x66e47b7956c2c, 0x34832fe1f55d0, 0x25c425e5d6263 ]),
            y_minus_x: Fe([ 0x4b3ae34dcb9ce, 0x47c691a15ac9f, 0x318e06e5d400c, 0x3c422d9f83eb1, 0x61545379465a6 ]),
            xy2d: Fe([ 0x606a6f1d7de6e, 0x4f1c0c46107e7, 0x229b1dcfbe5d8, 0x3acc60a7b1327, 0x6539a08915484 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4dbd414bb4a19, 0x7930849f1dbb8, 0x329c5a466caf0, 0x6c824544feb9b, 0xf65320ef019b ]),
            y_minus_x: Fe([ 0x21f74c3d2f773, 0x24b88d08bd3a, 0x6e678cf054151, 0x43631272e747c, 0x11c5e4aac5cd1 ]),
            xy2d: Fe([ 0x6d1b1cafde0c6, 0x462c76a303a90, 0x3ca4e693cff9b, 0x3952cd45786fd, 0x4cabc7bdec330 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x7788f3f78d289, 0x5942809b3f811, 0x5973277f8c29c, 0x10f93bc5fe67, 0x7ee498165acb2 ]),
            y_minus_x: Fe([ 0x69624089c0a2e, 0x75fc8e70473, 0x13e84ab1d2313, 0x2c10bedf6953b, 0x639b93f0321c8 ]),
            xy2d: Fe([ 0x508e39111a1c3, 0x290120e912f7a, 0x1cbf464acae43, 0x15373e9576157, 0xedf493c85b60 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7c4d284764113, 0x7fefebf06acec, 0x39afb7a824100, 0x1b48e47e7fd65, 0x4c00c54d1dfa ]),
            y_minus_x: Fe([ 0x48158599b5a68, 0x1fd75bc41d5d9, 0x2d9fc1fa95d3c, 0x7da27f20eba11, 0x403b92e3019d4 ]),
            xy2d: Fe([ 0x22f818b465cf8, 0x342901dff09b8, 0x31f595dc683cd, 0x37a57745fd682, 0x355bb12ab2617 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1dac75a8c7318, 0x3b679d5423460, 0x6b8fcb7b6400e, 0x6c73783be5f9d, 0x7518eaf8e052a ]),
            y_minus_x: Fe([ 0x664cc7493bbf4, 0x33d94761874e3, 0x179e1796f613, 0x1890535e2867d, 0xf9b8132182ec ]),
            xy2d: Fe([ 0x59c41b7f6c32, 0x79e8706531491, 0x6c747643cb582, 0x2e20c0ad494e4, 0x47c3871bbb175 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x65d50c85066b0, 0x6167453361f7c, 0x6ba3818bb312, 0x6aff29baa7522, 0x8fea02ce8d48 ]),
            y_minus_x: Fe([ 0x4539771ec4f48, 0x7b9318badca28, 0x70f19afe016c5, 0x4ee7bb1608d23, 0xb89b8576469 ]),
            xy2d: Fe([ 0x5dd7668deead0, 0x4096d0ba47049, 0x6275997219114, 0x29bda8a67e6ae, 0x473829a74f75d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1533aad3902c9, 0x1dde06b11e47b, 0x784bed1930b77, 0x1c80a92b9c867, 0x6c668b4d44e4d ]),
            y_minus_x: Fe([ 0x2da754679c418, 0x3164c31be105a, 0x11fac2b98ef5f, 0x35a1aaf779256, 0x2078684c4833c ]),
            xy2d: Fe([ 0xcf217a78820c, 0x65024e7d2e769, 0x23bb5efdda82a, 0x19fd4b632d3c6, 0x7411a6054f8a4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2e53d18b175b4, 0x33e7254204af3, 0x3bcd7d5a1c4c5, 0x4c7c22af65d0f, 0x1ec9a872458c3 ]),
            y_minus_x: Fe([ 0x59d32b99dc86d, 0x6ac075e22a9ac, 0x30b9220113371, 0x27fd9a638966e, 0x7c136574fb813 ]),
            xy2d: Fe([ 0x6a4d400a2509b, 0x41791056971c, 0x655d5866e075c, 0x2302bf3e64df8, 0x3add88a5c7cd6 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x298d459393046, 0x30bfecb3d90b8, 0x3d9b8ea3df8d6, 0x3900e96511579, 0x61ba1131a406a ]),
            y_minus_x: Fe([ 0x15770b635dcf2, 0x59ecd83f79571, 0x2db461c0b7fbd, 0x73a42a981345f, 0x249929fccc879 ]),
            xy2d: Fe([ 0xa0f116959029, 0x5974fd7b1347a, 0x1e0cc1c08edad, 0x673bdf8ad1f13, 0x5620310cbbd8e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6b5f477e285d6, 0x4ed91ec326cc8, 0x6d6537503a3fd, 0x626d3763988d5, 0x7ec846f3658ce ]),
            y_minus_x: Fe([ 0x193434934d643, 0xd4a2445eaa51, 0x7d0708ae76fe0, 0x39847b6c3c7e1, 0x37676a2a4d9d9 ]),
            xy2d: Fe([ 0x68f3f1da22ec7, 0x6ed8039a2736b, 0x2627ee04c3c75, 0x6ea90a647e7d1, 0x6daaf723399b9 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x304bfacad8ea2, 0x502917d108b07, 0x43176ca6dd0f, 0x5d5158f2c1d84, 0x2b5449e58eb3b ]),
            y_minus_x: Fe([ 0x27562eb3dbe47, 0x291d7b4170be7, 0x5d1ca67dfa8e1, 0x2a88061f298a2, 0x1304e9e71627d ]),
            xy2d: Fe([ 0x14d26adc9cfe, 0x7f1691ba16f13, 0x5e71828f06eac, 0x349ed07f0fffc, 0x4468de2d7c2dd ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2d8c6f86307ce, 0x6286ba1850973, 0x5e9dcb08444d4, 0x1a96a543362b2, 0x5da6427e63247 ]),
            y_minus_x: Fe([ 0x3355e9419469e, 0x1847bb8ea8a37, 0x1fe6588cf9b71, 0x6b1c9d2db6b22, 0x6cce7c6ffb44b ]),
            xy2d: Fe([ 0x4c688deac22ca, 0x6f775c3ff0352, 0x565603ee419bb, 0x6544456c61c46, 0x58f29abfe79f2 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x264bf710ecdf6, 0x708c58527896b, 0x42ceae6c53394, 0x4381b21e82b6a, 0x6af93724185b4 ]),
            y_minus_x: Fe([ 0x6cfab8de73e68, 0x3e6efced4bd21, 0x56609500dbe, 0x71b7824ad85df, 0x577629c4a7f41 ]),
            xy2d: Fe([ 0x24509c6a888, 0x2696ab12e6644, 0xcca27f4b80d8, 0xc7c1f11b119e, 0x701f25bb0caec ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xf6d97cbec113, 0x4ce97fb7c93a3, 0x139835a11281b, 0x728907ada9156, 0x720a5bc050955 ]),
            y_minus_x: Fe([ 0xb0f8e4616ced, 0x1d3c4b50fb875, 0x2f29673dc0198, 0x5f4b0f1830ffa, 0x2e0c92bfbdc40 ]),
            xy2d: Fe([ 0x709439b805a35, 0x6ec48557f8187, 0x8a4d1ba13a2c, 0x76348a0bf9ae, 0xe9b9cbb144ef ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x69bd55db1beee, 0x6e14e47f731bd, 0x1a35e47270eac, 0x66f225478df8e, 0x366d44191cfd3 ]),
            y_minus_x: Fe([ 0x2d48ffb5720ad, 0x57b7f21a1df77, 0x5550effba0645, 0x5ec6a4098a931, 0x221104eb3f337 ]),
            xy2d: Fe([ 0x41743f2bc8c14, 0x796b0ad8773c7, 0x29fee5cbb689b, 0x122665c178734, 0x4167a4e6bc593 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x62665f8ce8fee, 0x29d101ac59857, 0x4d93bbba59ffc, 0x17b7897373f17, 0x34b33370cb7ed ]),
            y_minus_x: Fe([ 0x39d2876f62700, 0x1cecd1d6c87, 0x7f01a11747675, 0x2350da5a18190, 0x7938bb7e22552 ]),
            xy2d: Fe([ 0x591ee8681d6cc, 0x39db0b4ea79b8, 0x202220f380842, 0x2f276ba42e0ac, 0x1176fc6e2dfe6 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xe28949770eb8, 0x5559e88147b72, 0x35e1e6e63ef30, 0x35b109aa7ff6f, 0x1f6a3e54f2690 ]),
            y_minus_x: Fe([ 0x76cd05b9c619b, 0x69654b0901695, 0x7a53710b77f27, 0x79a1ea7d28175, 0x8fc3a4c677d5 ]),
            xy2d: Fe([ 0x4c199d30734ea, 0x6c622cb9acc14, 0x5660a55030216, 0x68f1199f11fb, 0x4f2fad0116b90 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4d91db73bb638, 0x55f82538112c5, 0x6d85a279815de, 0x740b7b0cd9cf9, 0x3451995f2944e ]),
            y_minus_x: Fe([ 0x6b24194ae4e54, 0x2230afded8897, 0x23412617d5071, 0x3d5d30f35969b, 0x445484a4972ef ]),
            xy2d: Fe([ 0x2fcd09fea7d7c, 0x296126b9ed22a, 0x4a171012a05b2, 0x1db92c74d5523, 0x10b89ca604289 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x141be5a45f06e, 0x5adb38becaea7, 0x3fd46db41f2bb, 0x6d488bbb5ce39, 0x17d2d1d9ef0d4 ]),
            y_minus_x: Fe([ 0x147499718289c, 0xa48a67e4c7ab, 0x30fbc544bafe3, 0xc701315fe58a, 0x20b878d577b75 ]),
            xy2d: Fe([ 0x2af18073f3e6a, 0x33aea420d24fe, 0x298008bf4ff94, 0x3539171db961e, 0x72214f63cc65c ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5b7b9f43b29c9, 0x149ea31eea3b3, 0x4be7713581609, 0x2d87960395e98, 0x1f24ac855a154 ]),
            y_minus_x: Fe([ 0x37f405307a693, 0x2e5e66cf2b69c, 0x5d84266ae9c53, 0x5e4eb7de853b9, 0x5fdf48c58171c ]),
            xy2d: Fe([ 0x608328e9505aa, 0x22182841dc49a, 0x3ec96891d2307, 0x2f363fff22e03, 0xba739e2ae39 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x426f5ea88bb26, 0x33092e77f75c8, 0x1a53940d819e7, 0x1132e4f818613, 0x72297de7d518d ]),
            y_minus_x: Fe([ 0x698de5c8790d6, 0x268b8545beb25, 0x6d2648b96fedf, 0x47988ad1db07c, 0x3283a3e67ad7 ]),
            xy2d: Fe([ 0x41dc7be0cb939, 0x1b16c66100904, 0xa24c20cbc66d, 0x4a2e9efe48681, 0x5e1296846271 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7bbc8242c4550, 0x59a06103b35b7, 0x7237e4af32033, 0x726421ab3537a, 0x78cf25d38258c ]),
            y_minus_x: Fe([ 0x2eeb32d9c495a, 0x79e25772f9750, 0x6d747833bbf23, 0x6cdd816d5d749, 0x39c00c9c13698 ]),
            xy2d: Fe([ 0x66b8e31489d68, 0x573857e10e2b5, 0x13be816aa1472, 0x41964d3ad4bf8, 0x6b52076b3ff ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x37e16b9ce082d, 0x1882f57853eb9, 0x7d29eacd01fc5, 0x2e76a59b5e715, 0x7de2e9561a9f7 ]),
            y_minus_x: Fe([ 0xcfe19d95781c, 0x312cc621c453c, 0x145ace6da077c, 0x912bef9ce9b8, 0x4d57e3443bc76 ]),
            xy2d: Fe([ 0xd4f4b6a55ecb, 0x7ebb0bb733bce, 0x7ba6a05200549, 0x4f6ede4e22069, 0x6b2a90af1a602 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x3f3245bb2d80a, 0xe5f720f36efd, 0x3b9cccf60c06d, 0x84e323f37926, 0x465812c8276c2 ]),
            y_minus_x: Fe([ 0x3f4fc9ae61e97, 0x3bc07ebfa2d24, 0x3b744b55cd4a0, 0x72553b25721f3, 0x5fd8f4e9d12d3 ]),
            xy2d: Fe([ 0x3beb22a1062d9, 0x6a7063b82c9a8, 0xa5a35dc197ed, 0x3c80c06a53def, 0x5b32c2b1cb16 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4a42c7ad58195, 0x5c8667e799eff, 0x2e5e74c850a1, 0x3f0db614e869a, 0x31771a4856730 ]),
            y_minus_x: Fe([ 0x5eccd24da8fd, 0x580bbfdf07918, 0x7e73586873c6a, 0x74ceddf77f93e, 0x3b5556a37b471 ]),
            xy2d: Fe([ 0xc524e14dd482, 0x283457496c656, 0xad6bcfb6cd45, 0x375d1e8b02414, 0x4fc079d27a733 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x48b440c86c50d, 0x139929cca3b86, 0xf8f2e44cdf2f, 0x68432117ba6b2, 0x241170c2bae3c ]),
            y_minus_x: Fe([ 0x138b089bf2f7f, 0x4a05bfd34ea39, 0x203914c925ef5, 0x7497fffe04e3c, 0x124567cecaf98 ]),
            xy2d: Fe([ 0x1ab860ac473b4, 0x5c0227c86a7ff, 0x71b12bfc24477, 0x6a573a83075, 0x3f8612966c870 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0xfcfa36048d13, 0x66e7133bbb383, 0x64b42a8a45676, 0x4ea6e4f9a85cf, 0x26f57eee878a1 ]),
            y_minus_x: Fe([ 0x20cc9782a0dde, 0x65d4e3070aab3, 0x7bc8e31547736, 0x9ebfb1432d98, 0x504aa77679736 ]),
            xy2d: Fe([ 0x32cd55687efb1, 0x4448f5e2f6195, 0x568919d460345, 0x34c2e0ad1a27, 0x4041943d9dba3 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x17743a26caadd, 0x48c9156f9c964, 0x7ef278d1e9ad0, 0xce58ea7bd01, 0x12d931429800d ]),
            y_minus_x: Fe([ 0xeeba43ebcc96, 0x384dd5395f878, 0x1df331a35d272, 0x207ecfd4af70e, 0x1420a1d976843 ]),
            xy2d: Fe([ 0x67799d337594f, 0x1647548f6018, 0x57fce5578f145, 0x9220c142a71, 0x1b4f92314359a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x73030a49866b1, 0x2442be90b2679, 0x77bd3d8947dcf, 0x1fb55c1552028, 0x5ff191d56f9a2 ]),
            y_minus_x: Fe([ 0x4109d89150951, 0x225bd2d2d47cb, 0x57cc080e73bea, 0x6d71075721fcb, 0x239b572a7f132 ]),
            xy2d: Fe([ 0x6d433ac2d9068, 0x72bf930a47033, 0x64facf4a20ead, 0x365f7a2b9402a, 0x20c526a758f3 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1ef59f042cc89, 0x3b1c24976dd26, 0x31d665cb16272, 0x28656e470c557, 0x452cfe0a5602c ]),
            y_minus_x: Fe([ 0x34f89ed8dbbc, 0x73b8f948d8ef3, 0x786c1d323caab, 0x43bd4a9266e51, 0x2aacc4615313 ]),
            xy2d: Fe([ 0xf7a0647877df, 0x4e1cc0f93f0d4, 0x7ec4726ef1190, 0x3bdd58bf512f8, 0x4cfb7d7b304b8 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x699c29789ef12, 0x63beae321bc50, 0x325c340adbb35, 0x562e1a1e42bf6, 0x5b1d4cbc434d3 ]),
            y_minus_x: Fe([ 0x43d6cb89b75fe, 0x3338d5b900e56, 0x38d327d531a53, 0x1b25c61d51b9f, 0x14b4622b39075 ]),
            xy2d: Fe([ 0x32615cc0a9f26, 0x57711b99cb6df, 0x5a69c14e93c38, 0x6e88980a4c599, 0x2f98f71258592 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2ae444f54a701, 0x615397afbc5c2, 0x60d7783f3f8fb, 0x2aa675fc486ba, 0x1d8062e9e7614 ]),
            y_minus_x: Fe([ 0x4a74cb50f9e56, 0x531d1c2640192, 0xc03d9d6c7fd2, 0x57ccd156610c1, 0x3a6ae249d806a ]),
            xy2d: Fe([ 0x2da85a9907c5a, 0x6b23721ec4caf, 0x4d2d3a4683aa2, 0x7f9c6870efdef, 0x298b8ce8aef25 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x272ea0a2165de, 0x68179ef3ed06f, 0x4e2b9c0feac1e, 0x3ee290b1b63bb, 0x6ba6271803a7d ]),
            y_minus_x: Fe([ 0x27953eff70cb2, 0x54f22ae0ec552, 0x29f3da92e2724, 0x242ca0c22bd18, 0x34b8a8404d5ce ]),
            xy2d: Fe([ 0x6ecb583693335, 0x3ec76bfdfb84d, 0x2c895cf56a04f, 0x6355149d54d52, 0x71d62bdd465e1 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5b5dab1f75ef5, 0x1e2d60cbeb9a5, 0x527c2175dfe57, 0x59e8a2b8ff51f, 0x1c333621262b2 ]),
            y_minus_x: Fe([ 0x3cc28d378df80, 0x72141f4968ca6, 0x407696bdb6d0d, 0x5d271b22ffcfb, 0x74d5f317f3172 ]),
            xy2d: Fe([ 0x7e55467d9ca81, 0x6a5653186f50d, 0x6b188ece62df1, 0x4c66d36844971, 0x4aebcc4547e9d ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x8d9e7354b610, 0x26b750b6dc168, 0x162881e01acc9, 0x7966df31d01a5, 0x173bd9ddc9a1d ]),
            y_minus_x: Fe([ 0x71b276d01c9, 0xb0d8918e025e, 0x75beea79ee2eb, 0x3c92984094db8, 0x5d88fbf95a3db ]),
            xy2d: Fe([ 0xf1efe5872df, 0x5da872318256a, 0x59ceb81635960, 0x18cf37693c764, 0x6e1cd13b19ea ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x3af629e5b0353, 0x204f1a088e8e5, 0x10efc9ceea82e, 0x589863c2fa34b, 0x7f3a6a1a8d837 ]),
            y_minus_x: Fe([ 0xad516f166f23, 0x263f56d57c81a, 0x13422384638ca, 0x1331ff1af0a50, 0x3080603526e16 ]),
            xy2d: Fe([ 0x644395d3d800b, 0x2b9203dbedefc, 0x4b18ce656a355, 0x3f3466bc182c, 0x30d0fded2e513 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4971e68b84750, 0x52ccc9779f396, 0x3e904ae8255c8, 0x4ecae46f39339, 0x4615084351c58 ]),
            y_minus_x: Fe([ 0x14d1af21233b3, 0x1de1989b39c0b, 0x52669dc6f6f9e, 0x43434b28c3fc7, 0xa9214202c099 ]),
            xy2d: Fe([ 0x19c0aeb9a02e, 0x1a2c06995d792, 0x664cbb1571c44, 0x6ff0736fa80b2, 0x3bca0d2895ca5 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x8eb69ecc01bf, 0x5b4c8912df38d, 0x5ea7f8bc2f20e, 0x120e516caafaf, 0x4ea8b4038df28 ]),
            y_minus_x: Fe([ 0x31bc3c5d62a4, 0x7d9fe0f4c081e, 0x43ed51467f22c, 0x1e6cc0c1ed109, 0x5631deddae8f1 ]),
            xy2d: Fe([ 0x5460af1cad202, 0xb4919dd0655d, 0x7c4697d18c14c, 0x231c890bba2a4, 0x24ce0930542ca ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7a155fdf30b85, 0x1c6c6e5d487f9, 0x24be1134bdc5a, 0x1405970326f32, 0x549928a7324f4 ]),
            y_minus_x: Fe([ 0x90f5fd06c106, 0x6abb1021e43fd, 0x232bcfad711a0, 0x3a5c13c047f37, 0x41d4e3c28a06d ]),
            xy2d: Fe([ 0x632a763ee1a2e, 0x6fa4bffbd5e4d, 0x5fd35a6ba4792, 0x7b55e1de99de8, 0x491b66dec0dcf ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4a8ed0da64a1, 0x5ecfc45096ebe, 0x5edee93b488b2, 0x5b3c11a51bc8f, 0x4cf6b8b0b7018 ]),
            y_minus_x: Fe([ 0x5b13dc7ea32a7, 0x18fc2db73131e, 0x7e3651f8f57e3, 0x25656055fa965, 0x8f338d0c85ee ]),
            xy2d: Fe([ 0x3a821991a73bd, 0x3be6418f5870, 0x1ddc18eac9ef0, 0x54ce09e998dc2, 0x530d4a82eb078 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x173456c9abf9e, 0x7892015100dad, 0x33ee14095fecb, 0x6ad95d67a0964, 0xdb3e7e00cbfb ]),
            y_minus_x: Fe([ 0x43630e1f94825, 0x4d1956a6b4009, 0x213fe2df8b5e0, 0x5ce3a41191e6, 0x65ea753f10177 ]),
            xy2d: Fe([ 0x6fc3ee2096363, 0x7ec36b96d67ac, 0x510ec6a0758b1, 0xed87df022109, 0x2a4ec1921e1a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6162f1cf795f, 0x324ddcafe5eb9, 0x18d5e0463218, 0x7e78b9092428e, 0x36d12b5dec067 ]),
            y_minus_x: Fe([ 0x6259a3b24b8a2, 0x188b5f4170b9c, 0x681c0dee15deb, 0x4dfe665f37445, 0x3d143c5112780 ]),
            xy2d: Fe([ 0x5279179154557, 0x39f8f0741424d, 0x45e6eb357923d, 0x42c9b5edb746f, 0x2ef517885ba82 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x6bffb305b2f51, 0x5b112b2d712dd, 0x35774974fe4e2, 0x4af87a96e3a3, 0x57968290bb3a0 ]),
            y_minus_x: Fe([ 0x7974e8c58aedc, 0x7757e083488c6, 0x601c62ae7bc8b, 0x45370c2ecab74, 0x2f1b78fab143a ]),
            xy2d: Fe([ 0x2b8430a20e101, 0x1a49e1d88fee3, 0x38bbb47ce4d96, 0x1f0e7ba84d437, 0x7dc43e35dc2aa ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2a5c273e9718, 0x32bc9dfb28b4f, 0x48df4f8d5db1a, 0x54c87976c028f, 0x44fb81d82d50 ]),
            y_minus_x: Fe([ 0x66665887dd9c3, 0x629760a6ab0b2, 0x481e6c7243e6c, 0x97e37046fc77, 0x7ef72016758cc ]),
            xy2d: Fe([ 0x718c5a907e3d9, 0x3b9c98c6b383b, 0x6ed255eccdc, 0x6976538229a59, 0x7f79823f9c30d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x41ff068f587ba, 0x1c00a191bcd53, 0x7b56f9c209e25, 0x3781e5fccaabe, 0x64a9b0431c06d ]),
            y_minus_x: Fe([ 0x4d239a3b513e8, 0x29723f51b1066, 0x642f4cf04d9c3, 0x4da095aa09b7a, 0xa4e0373d784d ]),
            xy2d: Fe([ 0x3d6a15b7d2919, 0x41aa75046a5d6, 0x691751ec2d3da, 0x23638ab6721c4, 0x71a7d0ace183 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4355220e14431, 0xe1362a283981, 0x2757cd8359654, 0x2e9cd7ab10d90, 0x7c69bcf761775 ]),
            y_minus_x: Fe([ 0x72daac887ba0b, 0xb7f4ac5dda60, 0x3bdda2c0498a4, 0x74e67aa180160, 0x2c3bcc7146ea7 ]),
            xy2d: Fe([ 0xd7eb04e8295f, 0x4a5ea1e6fa0fe, 0x45e635c436c60, 0x28ef4a8d4d18b, 0x6f5a9a7322aca ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1d4eba3d944be, 0x100f15f3dce5, 0x61a700e367825, 0x5922292ab3d23, 0x2ab9680ee8d3 ]),
            y_minus_x: Fe([ 0x1000c2f41c6c5, 0x219fdf737174, 0x314727f127de7, 0x7e5277d23b81e, 0x494e21a2e147a ]),
            xy2d: Fe([ 0x48a85dde50d9a, 0x1c1f734493df4, 0x47bdb64866889, 0x59a7d048f8eec, 0x6b5d76cbea46b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x141171e782522, 0x6806d26da7c1f, 0x3f31d1bc79ab9, 0x9f20459f5168, 0x16fb869c03dd3 ]),
            y_minus_x: Fe([ 0x7556cec0cd994, 0x5eb9a03b7510a, 0x50ad1dd91cb71, 0x1aa5780b48a47, 0xae333f685277 ]),
            xy2d: Fe([ 0x6199733b60962, 0x69b157c266511, 0x64740f893f1ca, 0x3aa408fbf684, 0x3f81e38b8f70d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x37f355f17c824, 0x7ae85334815b, 0x7e3abddd2e48f, 0x61eeabe1f45e5, 0xad3e2d34cded ]),
            y_minus_x: Fe([ 0x10fcc7ed9affe, 0x4248cb0e96ff2, 0x4311c115172e2, 0x4c9d41cbf6925, 0x50510fc104f50 ]),
            xy2d: Fe([ 0x40fc5336e249d, 0x3386639fb2de1, 0x7bbf871d17b78, 0x75f796b7e8004, 0x127c158bf0fa1 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x28fc4ae51b974, 0x26e89bfd2dbd4, 0x4e122a07665cf, 0x7cab1203405c3, 0x4ed82479d167d ]),
            y_minus_x: Fe([ 0x17c422e9879a2, 0x28a5946c8fec3, 0x53ab32e912b77, 0x7b44da09fe0a5, 0x354ef87d07ef4 ]),
            xy2d: Fe([ 0x3b52260c5d975, 0x79d6836171fdc, 0x7d994f140d4bb, 0x1b6c404561854, 0x302d92d205392 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x46fb6e4e0f177, 0x53497ad5265b7, 0x1ebdba01386fc, 0x302f0cb36a3c, 0xedc5f5eb426d ]),
            y_minus_x: Fe([ 0x3c1a2bca4283d, 0x23430c7bb2f02, 0x1a3ea1bb58bc2, 0x7265763de5c61, 0x10e5d3b76f1ca ]),
            xy2d: Fe([ 0x3bfd653da8e67, 0x584953ec82a8a, 0x55e288fa7707b, 0x5395fc3931d81, 0x45b46c51361cb ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x54ddd8a7fe3e4, 0x2cecc41c619d3, 0x43a6562ac4d91, 0x4efa5aca7bdd9, 0x5c1c0aef32122 ]),
            y_minus_x: Fe([ 0x2abf314f7fa1, 0x391d19e8a1528, 0x6a2fa13895fc7, 0x9d8eddeaa591, 0x2177bfa36dcb7 ]),
            xy2d: Fe([ 0x1bbcfa79db8f, 0x3d84beb3666e1, 0x20c921d812204, 0x2dd843d3b32ce, 0x4ae619387d8ab ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x17e44985bfb83, 0x54e32c626cc22, 0x96412ff38118, 0x6b241d61a246a, 0x75685abe5ba43 ]),
            y_minus_x: Fe([ 0x3f6aa5344a32e, 0x69683680f11bb, 0x4c3581f623aa, 0x701af5875cba5, 0x1a00d91b17bf3 ]),
            xy2d: Fe([ 0x60933eb61f2b2, 0x5193fe92a4dd2, 0x3d995a550f43e, 0x3556fb93a883d, 0x135529b623b0e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x716bce22e83fe, 0x33d0130b83eb8, 0x952abad0afac, 0x309f64ed31b8a, 0x5972ea051590a ]),
            y_minus_x: Fe([ 0xdbd7add1d518, 0x119f823e2231e, 0x451d66e5e7de2, 0x500c39970f838, 0x79b5b81a65ca3 ]),
            xy2d: Fe([ 0x4ac20dc8f7811, 0x29589a9f501fa, 0x4d810d26a6b4a, 0x5ede00d96b259, 0x4f7e9c95905f3 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x443d355299fe, 0x39b7d7d5aee39, 0x692519a2f34ec, 0x6e4404924cf78, 0x1942eec4a144a ]),
            y_minus_x: Fe([ 0x74bbc5781302e, 0x73135bb81ec4c, 0x7ef671b61483c, 0x7264614ccd729, 0x31993ad92e638 ]),
            xy2d: Fe([ 0x45319ae234992, 0x2219d47d24fb5, 0x4f04488b06cf6, 0x53aaa9e724a12, 0x2a0a65314ef9c ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x61acd3c1c793a, 0x58b46b78779e6, 0x3369aacbe7af2, 0x509b0743074d4, 0x55dc39b6dea1 ]),
            y_minus_x: Fe([ 0x7937ff7f927c2, 0xc2fa14c6a5b6, 0x556bddb6dd07c, 0x6f6acc179d108, 0x4cf6e218647c2 ]),
            xy2d: Fe([ 0x1227cc28d5bb6, 0x78ee9bff57623, 0x28cb2241f893a, 0x25b541e3c6772, 0x121a307710aa2 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1713ec77483c9, 0x6f70572d5facb, 0x25ef34e22ff81, 0x54d944f141188, 0x527bb94a6ced3 ]),
            y_minus_x: Fe([ 0x35d5e9f034a97, 0x126069785bc9b, 0x5474ec7854ff0, 0x296a302a348ca, 0x333fc76c7a40e ]),
            xy2d: Fe([ 0x5992a995b482e, 0x78dc707002ac7, 0x5936394d01741, 0x4fba4281aef17, 0x6b89069b20a7a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2fa8cb5c7db77, 0x718e6982aa810, 0x39e95f81a1a1b, 0x5e794f3646cfb, 0x473d308a7639 ]),
            y_minus_x: Fe([ 0x2a0416270220d, 0x75f248b69d025, 0x1cbbc16656a27, 0x5b9ffd6e26728, 0x23bc2103aa73e ]),
            xy2d: Fe([ 0x6792603589e05, 0x248db9892595d, 0x6a53cad2d08, 0x20d0150f7ba73, 0x102f73bfde043 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x4dae0b5511c9a, 0x5257fffe0d456, 0x54108d1eb2180, 0x96cc0f9baefa, 0x3f6bd725da4ea ]),
            y_minus_x: Fe([ 0xb9ab7f5745c6, 0x5caf0f8d21d63, 0x7debea408ea2b, 0x9edb93896d16, 0x36597d25ea5c0 ]),
            xy2d: Fe([ 0x58d7b106058ac, 0x3cdf8d20bee69, 0xa4cb765015e, 0x36832337c7cc9, 0x7b7ecc19da60d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x64a51a77cfa9b, 0x29cf470ca0db5, 0x4b60b6e0898d9, 0x55d04ddffe6c7, 0x3bedc661bf5c ]),
            y_minus_x: Fe([ 0x2373c695c690d, 0x4c0c8520dcf18, 0x384af4b7494b9, 0x4ab4a8ea22225, 0x4235ad7601743 ]),
            xy2d: Fe([ 0xcb0d078975f5, 0x292313e530c4b, 0x38dbb9124a509, 0x350d0655a11f1, 0xe7ce2b0cdf06 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6fedfd94b70f9, 0x2383f9745bfd4, 0x4beae27c4c301, 0x75aa4416a3f3f, 0x615256138aece ]),
            y_minus_x: Fe([ 0x4643ac48c85a3, 0x6878c2735b892, 0x3a53523f4d877, 0x3a504ed8bee9d, 0x666e0a5d8fb46 ]),
            xy2d: Fe([ 0x3f64e4870cb0d, 0x61548b16d6557, 0x7a261773596f3, 0x7724d5f275d3a, 0x7f0bc810d514d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x49dad737213a0, 0x745dee5d31075, 0x7b1a55e7fdbe2, 0x5ba988f176ea1, 0x1d3a907ddec5a ]),
            y_minus_x: Fe([ 0x6ba426f4136f, 0x3cafc0606b720, 0x518f0a2359cda, 0x5fae5e46feca7, 0xd1f8dbcf8eed ]),
            xy2d: Fe([ 0x693313ed081dc, 0x5b0a366901742, 0x40c872ca4ca7e, 0x6f18094009e01, 0x11b44a31bf ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x61f696a0aa75c, 0x38b0a57ad42ca, 0x1e59ab706fdc9, 0x1308d46ebfcd, 0x63d988a2d2851 ]),
            y_minus_x: Fe([ 0x7a06c3fc66c0c, 0x1c9bac1ba47fb, 0x23935c575038e, 0x3f0bd71c59c13, 0x3ac48d916e835 ]),
            xy2d: Fe([ 0x20753afbd232e, 0x71fbb1ed06002, 0x39cae47a4af3a, 0x337c0b34d9c2, 0x33fad52b2368a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4c8d0c422cfe8, 0x760b4275971a5, 0x3da95bc1cad3d, 0xf151ff5b7376, 0x3cc355ccb90a7 ]),
            y_minus_x: Fe([ 0x649c6c5e41e16, 0x60667eee6aa80, 0x4179d182be190, 0x653d9567e6979, 0x16c0f429a256d ]),
            xy2d: Fe([ 0x69443903e9131, 0x16f4ac6f9dd36, 0x2ea4912e29253, 0x2b4643e68d25d, 0x631eaf426bae7 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x175b9a3700de8, 0x77c5f00aa48fb, 0x3917785ca0317, 0x5aa9b2c79399, 0x431f2c7f665f8 ]),
            y_minus_x: Fe([ 0x10410da66fe9f, 0x24d82dcb4d67d, 0x3e6fe0e17752d, 0x4dade1ecbb08f, 0x5599648b1ea91 ]),
            xy2d: Fe([ 0x26344858f7b19, 0x5f43d4a295ac0, 0x242a75c52acd4, 0x5934480220d10, 0x7b04715f91253 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6c280c4e6bac6, 0x3ada3b361766e, 0x42fe5125c3b4f, 0x111d84d4aac22, 0x48d0acfa57cde ]),
            y_minus_x: Fe([ 0x5bd28acf6ae43, 0x16fab8f56907d, 0x7acb11218d5f2, 0x41fe02023b4db, 0x59b37bf5c2f65 ]),
            xy2d: Fe([ 0x726e47dabe671, 0x2ec45e746f6c1, 0x6580e53c74686, 0x5eda104673f74, 0x16234191336d3 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x19cd61ff38640, 0x60c6c4b41ba9, 0x75cf70ca7366f, 0x118a8f16c011e, 0x4a25707a203b9 ]),
            y_minus_x: Fe([ 0x499def6267ff6, 0x76e858108773c, 0x693cac5ddcb29, 0x311d00a9ff4, 0x2cdfdfecd5d05 ]),
            xy2d: Fe([ 0x7668a53f6ed6a, 0x303ba2e142556, 0x3880584c10909, 0x4fe20000a261d, 0x5721896d248e4 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x55091a1d0da4e, 0x4f6bfc7c1050b, 0x64e4ecd2ea9be, 0x7eb1f28bbe70, 0x3c935afc4b03 ]),
            y_minus_x: Fe([ 0x65517fd181bae, 0x3e5772c76816d, 0x19189640898a, 0x1ed2a84de7499, 0x578edd74f63c1 ]),
            xy2d: Fe([ 0x276c6492b0c3d, 0x9bfc40bf932e, 0x588e8f11f330b, 0x3d16e694dc26e, 0x3ec2ab590288c ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x13a09ae32d1cb, 0x3e81eb85ab4e4, 0x7aaca43cae1f, 0x62f05d7526374, 0xe1bf66c6adba ]),
            y_minus_x: Fe([ 0xd27be4d87bb9, 0x56c27235db434, 0x72e6e0ea62d37, 0x5674cd06ee839, 0x2dd5c25a200fc ]),
            xy2d: Fe([ 0x3d5e9792c887e, 0x319724dabbc55, 0x2b97c78680800, 0x7afdfdd34e6dd, 0x730548b35ae88 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x3094ba1d6e334, 0x6e126a7e3300b, 0x89c0aefcfbc5, 0x2eea11f836583, 0x585a2277d8784 ]),
            y_minus_x: Fe([ 0x551a3cba8b8ee, 0x3b6422be2d886, 0x630e1419689bc, 0x4653b07a7a955, 0x3043443b411db ]),
            xy2d: Fe([ 0x25f8233d48962, 0x6bd8f04aff431, 0x4f907fd9a6312, 0x40fd3c737d29b, 0x7656278950ef9 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x73a3ea86cf9d, 0x6e0e2abfb9c2e, 0x60e2a38ea33ee, 0x30b2429f3fe18, 0x28bbf484b613f ]),
            y_minus_x: Fe([ 0x3cf59d51fc8c0, 0x7a0a0d6de4718, 0x55c3a3e6fb74b, 0x353135f884fd5, 0x3f4160a8c1b84 ]),
            xy2d: Fe([ 0x12f5c6f136c7c, 0xfedba237de4c, 0x779bccebfab44, 0x3aea93f4d6909, 0x1e79cb358188f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x153d8f5e08181, 0x8533bbdb2efd, 0x1149796129431, 0x17a6e36168643, 0x478ab52d39d1f ]),
            y_minus_x: Fe([ 0x436c3eef7e3f1, 0x7ffd3c21f0026, 0x3e77bf20a2da9, 0x418bffc8472de, 0x65d7951b3a3b3 ]),
            xy2d: Fe([ 0x6a4d39252d159, 0x790e35900ecd4, 0x30725bf977786, 0x10a5c1635a053, 0x16d87a411a212 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4d5e2d54e0583, 0x2e5d7b33f5f74, 0x3a5de3f887ebf, 0x6ef24bd6139b7, 0x1f990b577a5a6 ]),
            y_minus_x: Fe([ 0x57e5a42066215, 0x1a18b44983677, 0x3e652de1e6f8f, 0x6532be02ed8eb, 0x28f87c8165f38 ]),
            xy2d: Fe([ 0x44ead1be8f7d6, 0x5759d4f31f466, 0x378149f47943, 0x69f3be32b4f29, 0x45882fe1534d6 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x49929943c6fe4, 0x4347072545b15, 0x3226bced7e7c5, 0x3a134ced89df, 0x7dcf843ce405f ]),
            y_minus_x: Fe([ 0x1345d757983d6, 0x222f54234cccd, 0x1784a3d8adbb4, 0x36ebeee8c2bcc, 0x688fe5b8f626f ]),
            xy2d: Fe([ 0xd6484a4732c0, 0x7b94ac6532d92, 0x5771b8754850f, 0x48dd9df1461c8, 0x6739687e73271 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x5cc9dc80c1ac0, 0x683671486d4cd, 0x76f5f1a5e8173, 0x6d5d3f5f9df4a, 0x7da0b8f68d7e7 ]),
            y_minus_x: Fe([ 0x2014385675a6, 0x6155fb53d1def, 0x37ea32e89927c, 0x59a668f5a82e, 0x46115aba1d4dc ]),
            xy2d: Fe([ 0x71953c3b5da76, 0x6642233d37a81, 0x2c9658076b1bd, 0x5a581e63010ff, 0x5a5f887e83674 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x628d3a0a643b9, 0x1cd8640c93d2, 0xb7b0cad70f2c, 0x3864da98144be, 0x43e37ae2d5d1c ]),
            y_minus_x: Fe([ 0x301cf70a13d11, 0x2a6a1ba1891ec, 0x2f291fb3f3ae0, 0x21a7b814bea52, 0x3669b656e44d1 ]),
            xy2d: Fe([ 0x63f06eda6e133, 0x233342758070f, 0x98e0459cc075, 0x4df5ead6c7c1b, 0x6a21e6cd4fd5e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x129126699b2e3, 0xee11a2603de8, 0x60ac2f5c74c21, 0x59b192a196808, 0x45371b07001e8 ]),
            y_minus_x: Fe([ 0x6170a3046e65f, 0x5401a46a49e38, 0x20add5561c4a8, 0x7abb4edde9e46, 0x586bf9f1a195f ]),
            xy2d: Fe([ 0x3088d5ef8790b, 0x38c2126fcb4db, 0x685bae149e3c3, 0xbcd601a4e930, 0xeafb03790e52 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x805e0f75ae1d, 0x464cc59860a28, 0x248e5b7b00bef, 0x5d99675ef8f75, 0x44ae3344c5435 ]),
            y_minus_x: Fe([ 0x555c13748042f, 0x4d041754232c0, 0x521b430866907, 0x3308e40fb9c39, 0x309acc675a02c ]),
            xy2d: Fe([ 0x289b9bba543ee, 0x3ab592e28539e, 0x64d82abcdd83a, 0x3c78ec172e327, 0x62d5221b7f946 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5d4263af77a3c, 0x23fdd2289aeb0, 0x7dc64f77eb9ec, 0x1bd28338402c, 0x14f29a5383922 ]),
            y_minus_x: Fe([ 0x4299c18d0936d, 0x5914183418a49, 0x52a18c721aed5, 0x2b151ba82976d, 0x5c0efde4bc754 ]),
            xy2d: Fe([ 0x17edc25b2d7f5, 0x37336a6081bee, 0x7b5318887e5c3, 0x49f6d491a5be1, 0x5e72365c7bee0 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x339062f08b33e, 0x4bbf3e657cfb2, 0x67af7f56e5967, 0x4dbd67f9ed68f, 0x70b20555cb734 ]),
            y_minus_x: Fe([ 0x3fc074571217f, 0x3a0d29b2b6aeb, 0x6478ccdde59d, 0x55e4d051bddfa, 0x77f1104c47b4e ]),
            xy2d: Fe([ 0x113c555112c4c, 0x7535103f9b7ca, 0x140ed1d9a2108, 0x2522333bc2af, 0xe34398f4a064 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x30b093e4b1928, 0x1ce7e7ec80312, 0x4e575bdf78f84, 0x61f7a190bed39, 0x6f8aded6ca379 ]),
            y_minus_x: Fe([ 0x522d93ecebde8, 0x24f045e0f6cf, 0x16db63426cfa1, 0x1b93a1fd30fd8, 0x5e5405368a362 ]),
            xy2d: Fe([ 0x123dfdb7b29a, 0x4344356523c68, 0x79a527921ee5f, 0x74bfccb3e817e, 0x780de72ec8d3d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7eaf300f42772, 0x5455188354ce3, 0x4dcca4a3dcbac, 0x3d314d0bfebcb, 0x1defc6ad32b58 ]),
            y_minus_x: Fe([ 0x28545089ae7bc, 0x1e38fe9a0c15c, 0x12046e0e2377b, 0x6721c560aa885, 0xeb28bf671928 ]),
            xy2d: Fe([ 0x3be1aef5195a7, 0x6f22f62bdb5eb, 0x39768b8523049, 0x43394c8fbfdbd, 0x467d201bf8dd2 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x6f4bd567ae7a9, 0x65ac89317b783, 0x7d3b20fd8932, 0xf208326916, 0x2ef9c5a5ba384 ]),
            y_minus_x: Fe([ 0x6919a74ef4fad, 0x59ed4611452bf, 0x691ec04ea09ef, 0x3cbcb2700e984, 0x71c43c4f5ba3c ]),
            xy2d: Fe([ 0x56df6fa9e74cd, 0x79c95e4cf56df, 0x7be643bc609e2, 0x149c12ad9e878, 0x5a758ca390c5f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x918b1d61dc94, 0xd350260cd19c, 0x7a2ab4e37b4d9, 0x21fea735414d7, 0xa738027f639d ]),
            y_minus_x: Fe([ 0x72710d9462495, 0x25aafaa007456, 0x2d21f28eaa31b, 0x17671ea005fd0, 0x2dbae244b3eb7 ]),
            xy2d: Fe([ 0x74a2f57ffe1cc, 0x1bc3073087301, 0x7ec57f4019c34, 0x34e082e1fa524, 0x2698ca635126a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5702f5e3dd90e, 0x31c9a4a70c5c7, 0x136a5aa78fc24, 0x1992f3b9f7b01, 0x3c004b0c4afa3 ]),
            y_minus_x: Fe([ 0x5318832b0ba78, 0x6f24b9ff17cec, 0xa47f30e060c7, 0x58384540dc8d0, 0x1fb43dcc49cae ]),
            xy2d: Fe([ 0x146ac06f4b82b, 0x4b500d89e7355, 0x3351e1c728a12, 0x10b9f69932fe3, 0x6b43fd01cd1fd ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x742583e760ef3, 0x73dc1573216b8, 0x4ae48fdd7714a, 0x4f85f8a13e103, 0x73420b2d6ff0d ]),
            y_minus_x: Fe([ 0x75d4b4697c544, 0x11be1fff7f8f4, 0x119e16857f7e1, 0x38a14345cf5d5, 0x5a68d7105b52f ]),
            xy2d: Fe([ 0x4f6cb9e851e06, 0x278c4471895e5, 0x7efcdce3d64e4, 0x64f6d455c4b4c, 0x3db5632fea34b ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x190b1829825d5, 0xe7d3513225c9, 0x1c12be3b7abae, 0x58777781e9ca6, 0x59197ea495df2 ]),
            y_minus_x: Fe([ 0x6ee2bf75dd9d8, 0x6c72ceb34be8d, 0x679c9cc345ec7, 0x7898df96898a4, 0x4321adf49d75 ]),
            xy2d: Fe([ 0x16019e4e55aae, 0x74fc5f25d209c, 0x4566a939ded0d, 0x66063e716e0b7, 0x45eafdc1f4d70 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x64624cfccb1ed, 0x257ab8072b6c1, 0x120725676f0a, 0x4a018d04e8eee, 0x3f73ceea5d56d ]),
            y_minus_x: Fe([ 0x401858045d72b, 0x459e5e0ca2d30, 0x488b719308bea, 0x56f4a0d1b32b5, 0x5a5eebc80362d ]),
            xy2d: Fe([ 0x7bfd10a4e8dc6, 0x7c899366736f4, 0x55ebbeaf95c01, 0x46db060903f8a, 0x2605889126621 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x18e3cc676e542, 0x26079d995a990, 0x4a7c217908b2, 0x1dc7603e6655a, 0xdedfa10b2444 ]),
            y_minus_x: Fe([ 0x704a68360ff04, 0x3cecc3cde8b3e, 0x21cd5470f64ff, 0x6abc18d953989, 0x54ad0c2e4e615 ]),
            xy2d: Fe([ 0x367d5b82b522a, 0xd3f4b83d7dc7, 0x3067f4cdbc58d, 0x20452da697937, 0x62ecb2baa77a9 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x72836afb62874, 0xaf3c2094b240, 0xc285297f357a, 0x7cc2d5680d6e3, 0x61913d5075663 ]),
            y_minus_x: Fe([ 0x5795261152b3d, 0x7a1dbbafa3cbd, 0x5ad31c52588d5, 0x45f3a4164685c, 0x2e59f919a966d ]),
            xy2d: Fe([ 0x62d361a3231da, 0x65284004e01b8, 0x656533be91d60, 0x6ae016c00a89f, 0x3ddbc2a131c05 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x257a22796bb14, 0x6f360fb443e75, 0x680e47220eaea, 0x2fcf2a5f10c18, 0x5ee7fb38d8320 ]),
            y_minus_x: Fe([ 0x40ff9ce5ec54b, 0x57185e261b35b, 0x3e254540e70a9, 0x1b5814003e3f8, 0x78968314ac04b ]),
            xy2d: Fe([ 0x5fdcb41446a8e, 0x5286926ff2a71, 0xf231e296b3f6, 0x684a357c84693, 0x61d0633c9bca0 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x328bcf8fc73df, 0x3b4de06ff95b4, 0x30aa427ba11a5, 0x5ee31bfda6d9c, 0x5b23ac2df8067 ]),
            y_minus_x: Fe([ 0x44935ffdb2566, 0x12f016d176c6e, 0x4fbb00f16f5ae, 0x3fab78d99402a, 0x6e965fd847aed ]),
            xy2d: Fe([ 0x2b953ee80527b, 0x55f5bcdb1b35a, 0x43a0b3fa23c66, 0x76e07388b820a, 0x79b9bbb9dd95d ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x17dae8e9f7374, 0x719f76102da33, 0x5117c2a80ca8b, 0x41a66b65d0936, 0x1ba811460accb ]),
            y_minus_x: Fe([ 0x355406a3126c2, 0x50d1918727d76, 0x6e5ea0b498e0e, 0xa3b6063214f2, 0x5065f158c9fd2 ]),
            xy2d: Fe([ 0x169fb0c429954, 0x59aedd9ecee10, 0x39916eb851802, 0x57917555cc538, 0x3981f39e58a4f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5dfa56de66fde, 0x58809075908, 0x6d3d8cb854a94, 0x5b2f4e970b1e3, 0x30f4452edcbc1 ]),
            y_minus_x: Fe([ 0x38a7559230a93, 0x52c1cde8ba31f, 0x2a4f2d4745a3d, 0x7e9d42d4a28a, 0x38dc083705acd ]),
            xy2d: Fe([ 0x52782c5759740, 0x53f3397d990ad, 0x3a939c7e84d15, 0x234c4227e39e0, 0x632d9a1a593f2 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1fd11ed0c84a7, 0x21b3ed2757e1, 0x73e1de58fc1c6, 0x5d110c84616ab, 0x3a5a7df28af64 ]),
            y_minus_x: Fe([ 0x36b15b807cba6, 0x3f78a9e1afed7, 0xa59c2c608f1f, 0x52bdd8ecb81b7, 0xb24f48847ed4 ]),
            xy2d: Fe([ 0x2d4be511beac7, 0x6bda4d99e5b9b, 0x17e6996914e01, 0x7b1f0ce7fcf80, 0x34fcf74475481 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x31dab78cfaa98, 0x4e3216e5e54b7, 0x249823973b689, 0x2584984e48885, 0x119a3042fb37 ]),
            y_minus_x: Fe([ 0x7e04c789767ca, 0x1671b28cfb832, 0x7e57ea2e1c537, 0x1fbaaef444141, 0x3d3bdc164dfa6 ]),
            xy2d: Fe([ 0x2d89ce8c2177d, 0x6cd12ba182cf4, 0x20a8ac19a7697, 0x539fab2cc72d9, 0x56c088f1ede20 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x35fac24f38f02, 0x7d75c6197ab03, 0x33e4bc2a42fa7, 0x1c7cd10b48145, 0x38b7ea483590 ]),
            y_minus_x: Fe([ 0x53d1110a86e17, 0x6416eb65f466d, 0x41ca6235fce20, 0x5c3fc8a99bb12, 0x9674c6b99108 ]),
            xy2d: Fe([ 0x6f82199316ff8, 0x5d54f1a9f3e9, 0x3bcc5d0bd274a, 0x5b284b8d2d5ad, 0x6e5e31025969e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4fb0e63066222, 0x130f59747e660, 0x41868fecd41a, 0x3105e8c923bc6, 0x3058ad43d1838 ]),
            y_minus_x: Fe([ 0x462f587e593fb, 0x3d94ba7ce362d, 0x330f9b52667b7, 0x5d45a48e0f00a, 0x8f5114789a8d ]),
            xy2d: Fe([ 0x40ffde57663d0, 0x71445d4c20647, 0x2653e68170f7c, 0x64cdee3c55ed6, 0x26549fa4efe3d ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x68549af3f666e, 0x9e2941d4bb68, 0x2e8311f5dff3c, 0x6429ef91ffbd2, 0x3a10dfe132ce3 ]),
            y_minus_x: Fe([ 0x55a461e6bf9d6, 0x78eeef4b02e83, 0x1d34f648c16cf, 0x7fea2aba5132, 0x1926e1dc6401e ]),
            xy2d: Fe([ 0x74e8aea17cea0, 0xc743f83fbc0f, 0x7cb03c4bf5455, 0x68a8ba9917e98, 0x1fa1d01d861e5 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4ac00d1df94ab, 0x3ba2101bd271b, 0x7578988b9c4af, 0xf2bf89f49f7e, 0x73fced18ee9a0 ]),
            y_minus_x: Fe([ 0x55947d599832, 0x346fe2aa41990, 0x164c8079195b, 0x799ccfb7bba27, 0x773563bc6a75c ]),
            xy2d: Fe([ 0x1e90863139cb3, 0x4f8b407d9a0d6, 0x58e24ca924f69, 0x7a246bbe76456, 0x1f426b701b864 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x635c891a12552, 0x26aebd38ede2f, 0x66dc8faddae05, 0x21c7d41a03786, 0xb76bb1b3fa7e ]),
            y_minus_x: Fe([ 0x1264c41911c01, 0x702f44584bdf9, 0x43c511fc68ede, 0x482c3aed35f9, 0x4e1af5271d31b ]),
            xy2d: Fe([ 0xc1f97f92939b, 0x17a88956dc117, 0x6ee005ef99dc7, 0x4aa9172b231cc, 0x7b6dd61eb772a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xabf9ab01d2c7, 0x3880287630ae6, 0x32eca045beddb, 0x57f43365f32d0, 0x53fa9b659bff6 ]),
            y_minus_x: Fe([ 0x5c1e850f33d92, 0x1ec119ab9f6f5, 0x7f16f6de663e9, 0x7a7d6cb16dec6, 0x703e9bceaf1d2 ]),
            xy2d: Fe([ 0x4c8e994885455, 0x4ccb5da9cad82, 0x3596bc610e975, 0x7a80c0ddb9f5e, 0x398d93e5c4c61 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x77c60d2e7e3f2, 0x4061051763870, 0x67bc4e0ecd2aa, 0x2bb941f1373b9, 0x699c9c9002c30 ]),
            y_minus_x: Fe([ 0x3d16733e248f3, 0xe2b7e14be389, 0x42c0ddaf6784a, 0x589ea1fc67850, 0x53b09b5ddf191 ]),
            xy2d: Fe([ 0x6a7235946f1cc, 0x6b99cbb2fbe60, 0x6d3a5d6485c62, 0x4839466e923c0, 0x51caf30c6fcdd ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2f99a18ac54c7, 0x398a39661ee6f, 0x384331e40cde3, 0x4cd15c4de19a6, 0x12ae29c189f8e ]),
            y_minus_x: Fe([ 0x3a7427674e00a, 0x6142f4f7e74c1, 0x4cc93318c3a15, 0x6d51bac2b1ee7, 0x5504aa292383f ]),
            xy2d: Fe([ 0x6c0cb1f0d01cf, 0x187469ef5d533, 0x27138883747bf, 0x2f52ae53a90e8, 0x5fd14fe958eba ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2fe5ebf93cb8e, 0x226da8acbe788, 0x10883a2fb7ea1, 0x94707842cf44, 0x7dd73f960725d ]),
            y_minus_x: Fe([ 0x42ddf2845ab2c, 0x6214ffd3276bb, 0xb8d181a5246, 0x268a6d579eb20, 0x93ff26e58647 ]),
            xy2d: Fe([ 0x524fe68059829, 0x65b75e47cb621, 0x15eb0a5d5cc19, 0x5209b3929d5a, 0x2f59bcbc86b47 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x1d560b691c301, 0x7f5bafce3ce08, 0x4cd561614806c, 0x4588b6170b188, 0x2aa55e3d01082 ]),
            y_minus_x: Fe([ 0x47d429917135f, 0x3eacfa07af070, 0x1deab46b46e44, 0x7a53f3ba46cdf, 0x5458b42e2e51a ]),
            xy2d: Fe([ 0x192e60c07444f, 0x5ae8843a21daa, 0x6d721910b1538, 0x3321a95a6417e, 0x13e9004a8a768 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x600c9193b877f, 0x21c1b8a0d7765, 0x379927fb38ea2, 0x70d7679dbe01b, 0x5f46040898de9 ]),
            y_minus_x: Fe([ 0x58845832fcedb, 0x135cd7f0c6e73, 0x53ffbdfe8e35b, 0x22f195e06e55b, 0x73937e8814bce ]),
            xy2d: Fe([ 0x37116297bf48d, 0x45a9e0d069720, 0x25af71aa744ec, 0x41af0cb8aaba3, 0x2cf8a4e891d5e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x5487e17d06ba2, 0x3872a032d6596, 0x65e28c09348e0, 0x27b6bb2ce40c2, 0x7a6f7f2891d6a ]),
            y_minus_x: Fe([ 0x3fd8707110f67, 0x26f8716a92db2, 0x1cdaa1b753027, 0x504be58b52661, 0x2049bd6e58252 ]),
            xy2d: Fe([ 0x1fd8d6a9aef49, 0x7cb67b7216fa1, 0x67aff53c3b982, 0x20ea610da9628, 0x6011aadfc5459 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6d0c802cbf890, 0x141bfed554c7b, 0x6dbb667ef4263, 0x58f3126857edc, 0x69ce18b779340 ]),
            y_minus_x: Fe([ 0x7926dcf95f83c, 0x42e25120e2bec, 0x63de96df1fa15, 0x4f06b50f3f9cc, 0x6fc5cc1b0b62f ]),
            xy2d: Fe([ 0x75528b29879cb, 0x79a8fd2125a3d, 0x27c8d4b746ab8, 0xf8893f02210c, 0x15596b3ae5710 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x731167e5124ca, 0x17b38e8bbe13f, 0x3d55b942f9056, 0x9c1495be913f, 0x3aa4e241afb6d ]),
            y_minus_x: Fe([ 0x739d23f9179a2, 0x632fadbb9e8c4, 0x7c8522bfe0c48, 0x6ed0983ef5aa9, 0xd2237687b5f4 ]),
            xy2d: Fe([ 0x138bf2a3305f5, 0x1f45d24d86598, 0x5274bad2160fe, 0x1b6041d58d12a, 0x32fcaa6e4687a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7a4732787ccdf, 0x11e427c7f0640, 0x3659385f8c64, 0x5f4ead9766bfb, 0x746f6336c2600 ]),
            y_minus_x: Fe([ 0x56e8dc57d9af5, 0x5b3be17be4f78, 0x3bf928cf82f4b, 0x52e55600a6f11, 0x4627e9cefebd6 ]),
            xy2d: Fe([ 0x2f345ab6c971c, 0x653286e63e7e9, 0x51061b78a23ad, 0x14999acb54501, 0x7b4917007ed66 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x41b28dd53a2dd, 0x37be85f87ea86, 0x74be3d2a85e41, 0x1be87fac96ca6, 0x1d03620fe08cd ]),
            y_minus_x: Fe([ 0x5fb5cab84b064, 0x2513e778285b0, 0x457383125e043, 0x6bda3b56e223d, 0x122ba376f844f ]),
            xy2d: Fe([ 0x232cda2b4e554, 0x422ba30ff840, 0x751e7667b43f5, 0x6261755da5f3e, 0x2c70bf52b68e ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x532bf458d72e1, 0x40f96e796b59c, 0x22ef79d6f9da3, 0x501ab67beca77, 0x6b0697e3feb43 ]),
            y_minus_x: Fe([ 0x7ec4b5d0b2fbb, 0x200e910595450, 0x742057105715e, 0x2f07022530f60, 0x26334f0a409ef ]),
            xy2d: Fe([ 0xf04adf62a3c0, 0x5e0edb48bb6d9, 0x7c34aa4fbc003, 0x7d74e4e5cac24, 0x1cc37f43441b2 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x656f1c9ceaeb9, 0x7031cacad5aec, 0x1308cd0716c57, 0x41c1373941942, 0x3a346f772f196 ]),
            y_minus_x: Fe([ 0x7565a5cc7324f, 0x1ca0d5244a11, 0x116b067418713, 0xa57d8c55edae, 0x6c6809c103803 ]),
            xy2d: Fe([ 0x55112e2da6ac8, 0x6363d0a3dba5a, 0x319c98ba6f40c, 0x2e84b03a36ec7, 0x5911b9f6ef7c ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x1acf3512eeaef, 0x2639839692a69, 0x669a234830507, 0x68b920c0603d4, 0x555ef9d1c64b2 ]),
            y_minus_x: Fe([ 0x39983f5df0ebb, 0x1ea2589959826, 0x6ce638703cdd6, 0x6311678898505, 0x6b3cecf9aa270 ]),
            xy2d: Fe([ 0x770ba3b73bd08, 0x11475f7e186d4, 0x251bc9892bbc, 0x24eab9bffcc5a, 0x675f4de133817 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7f6d93bdab31d, 0x1f3aca5bfd425, 0x2fa521c1c9760, 0x62180ce27f9cd, 0x60f450b882cd3 ]),
            y_minus_x: Fe([ 0x452036b1782fc, 0x2d95b07681c5, 0x5901cf99205b2, 0x290686e5eecb4, 0x13d99df70164c ]),
            xy2d: Fe([ 0x35ec321e5c0ca, 0x13ae337f44029, 0x4008e813f2da7, 0x640272f8e0c3a, 0x1c06de9e55eda ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x52b40ff6d69aa, 0x31b8809377ffa, 0x536625cd14c2c, 0x516af252e17d1, 0x78096f8e7d32b ]),
            y_minus_x: Fe([ 0x77ad6a33ec4e2, 0x717c5dc11d321, 0x4a114559823e4, 0x306ce50a1e2b1, 0x4cf38a1fec2db ]),
            xy2d: Fe([ 0x2aa650dfa5ce7, 0x54916a8f19415, 0xdc96fe71278, 0x55f2784e63eb8, 0x373cad3a26091 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6a8fb89ddbbad, 0x78c35d5d97e37, 0x66e3674ef2cb2, 0x34347ac53dd8f, 0x21547eda5112a ]),
            y_minus_x: Fe([ 0x4634d82c9f57c, 0x4249268a6d652, 0x6336d687f2ff7, 0x4fe4f4e26d9a0, 0x40f3d945441 ]),
            xy2d: Fe([ 0x5e939fd5986d3, 0x12a2147019bdf, 0x4c466e7d09cb2, 0x6fa5b95d203dd, 0x63550a334a254 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2584572547b49, 0x75c58811c1377, 0x4d3c637cc171b, 0x33d30747d34e3, 0x39a92bafaa7d7 ]),
            y_minus_x: Fe([ 0x7d6edb569cf37, 0x60194a5dc2ca0, 0x5af59745e10a6, 0x7a8f53e004875, 0x3eea62c7daf78 ]),
            xy2d: Fe([ 0x4c713e693274e, 0x6ed1b7a6eb3a4, 0x62ace697d8e15, 0x266b8292ab075, 0x68436a0665c9c ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x6d317e820107c, 0x90815d2ca3ca, 0x3ff1eb1499a1, 0x23960f050e319, 0x5373669c91611 ]),
            y_minus_x: Fe([ 0x235e8202f3f27, 0x44c9f2eb61780, 0x630905b1d7003, 0x4fcc8d274ead1, 0x17b6e7f68ab78 ]),
            xy2d: Fe([ 0x14ab9a0e5257, 0x9939567f8ba5, 0x4b47b2a423c82, 0x688d7e57ac42d, 0x1cb4b5a678f87 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4aa62a2a007e7, 0x61e0e38f62d6e, 0x2f888fcc4782, 0x7562b83f21c00, 0x2dc0fd2d82ef6 ]),
            y_minus_x: Fe([ 0x4c06b394afc6c, 0x4931b4bf636cc, 0x72b60d0322378, 0x25127c6818b25, 0x330bca78de743 ]),
            xy2d: Fe([ 0x6ff841119744e, 0x2c560e8e49305, 0x7254fefe5a57a, 0x67ae2c560a7df, 0x3c31be1b369f1 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0xbc93f9cb4272, 0x3f8f9db73182d, 0x2b235eabae1c4, 0x2ddbf8729551a, 0x41cec1097e7d5 ]),
            y_minus_x: Fe([ 0x4864d08948aee, 0x5d237438df61e, 0x2b285601f7067, 0x25dbcbae6d753, 0x330b61134262d ]),
            xy2d: Fe([ 0x619d7a26d808a, 0x3c3b3c2adbef2, 0x6877c9eec7f52, 0x3beb9ebe1b66d, 0x26b44cd91f287 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x7f29362730383, 0x7fd7951459c36, 0x7504c512d49e7, 0x87ed7e3bc55f, 0x7deb10149c726 ]),
            y_minus_x: Fe([ 0x48478f387475, 0x69397d9678a3e, 0x67c8156c976f3, 0x2eb4d5589226c, 0x2c709e6c1c10a ]),
            xy2d: Fe([ 0x2af6a8766ee7a, 0x8aaa79a1d96c, 0x42f92d59b2fb0, 0x1752c40009c07, 0x8e68e9ff62ce ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x509d50ab8f2f9, 0x1b8ab247be5e5, 0x5d9b2e6b2e486, 0x4faa5479a1339, 0x4cb13bd738f71 ]),
            y_minus_x: Fe([ 0x5500a4bc130ad, 0x127a17a938695, 0x2a26fa34e36d, 0x584d12e1ecc28, 0x2f1f3f87eeba3 ]),
            xy2d: Fe([ 0x48c75e515b64a, 0x75b6952071ef0, 0x5d46d42965406, 0x7746106989f9f, 0x19a1e353c0ae2 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x172cdd596bdbd, 0x731ddf881684, 0x10426d64f8115, 0x71a4fd8a9a3da, 0x736bd3990266a ]),
            y_minus_x: Fe([ 0x47560bafa05c3, 0x418dcabcc2fa3, 0x35991cecf8682, 0x24371a94b8c60, 0x41546b11c20c3 ]),
            xy2d: Fe([ 0x32d509334b3b4, 0x16c102cae70aa, 0x1720dd51bf445, 0x5ae662faf9821, 0x412295a2b87fa ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x55261e293eac6, 0x6426759b65cc, 0x40265ae116a48, 0x6c02304bae5bc, 0x760bb8d195ad ]),
            y_minus_x: Fe([ 0x19b88f57ed6e9, 0x4cdbf1904a339, 0x42b49cd4e4f2c, 0x71a2e771909d9, 0x14e153ebb52d2 ]),
            xy2d: Fe([ 0x61a17cde6818a, 0x53dad34108827, 0x32b32c55c55b6, 0x2f9165f9347a3, 0x6b34be9bc33ac ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x469656571f2d3, 0xaa61ce6f423f, 0x3f940d71b27a1, 0x185f19d73d16a, 0x1b9c7b62e6dd ]),
            y_minus_x: Fe([ 0x72f643a78c0b2, 0x3de45c04f9e7b, 0x706d68d30fa5c, 0x696f63e8e2f24, 0x2012c18f0922d ]),
            xy2d: Fe([ 0x355e55ac89d29, 0x3e8b414ec7101, 0x39db07c520c90, 0x6f41e9b77efe1, 0x8af5b784e4ba ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x314d289cc2c4b, 0x23450e2f1bc4e, 0xcd93392f92f4, 0x1370c6a946b7d, 0x6423c1d5afd98 ]),
            y_minus_x: Fe([ 0x499dc881f2533, 0x34ef26476c506, 0x4d107d2741497, 0x346c4bd6efdb3, 0x32b79d71163a1 ]),
            xy2d: Fe([ 0x5f8d9edfcb36a, 0x1e6e8dcbf3990, 0x7974f348af30a, 0x6e6724ef19c7c, 0x480a5efbc13e2 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x14ce442ce221f, 0x18980a72516cc, 0x72f80db86677, 0x703331fda526e, 0x24b31d47691c8 ]),
            y_minus_x: Fe([ 0x1e70b01622071, 0x1f163b5f8a16a, 0x56aaf341ad417, 0x7989635d830f7, 0x47aa27600cb7b ]),
            xy2d: Fe([ 0x41eedc015f8c3, 0x7cf8d27ef854a, 0x289e3584693f9, 0x4a7857b309a7, 0x545b585d14dda ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x4e4d0e3b321e1, 0x7451fe3d2ac40, 0x666f678eea98d, 0x38858667fead, 0x4d22dc3e64c8d ]),
            y_minus_x: Fe([ 0x7275ea0d43a0f, 0x681137dd7ccf7, 0x1e79cbab79a38, 0x22a214489a66a, 0xf62f9c332ba5 ]),
            xy2d: Fe([ 0x46589d63b5f39, 0x7eaf979ec3f96, 0x4ebe81572b9a8, 0x21b7f5d61694a, 0x1c0fa01a36371 ]),
        },
    ],
    [
        GePrecomp {
            y_plus_x: Fe([ 0x2b0e8c936a50, 0x6b83b58b6cd21, 0x37ed8d3e72680, 0xa037db9f2a62, 0x4005419b1d2bc ]),
            y_minus_x: Fe([ 0x604b622943dff, 0x1c899f6741a58, 0x60219e2f232fb, 0x35fae92a7f9cb, 0xfa3614f3b1ca ]),
            xy2d: Fe([ 0x3febdb9be82f0, 0x5e74895921400, 0x553ea38822706, 0x5a17c24cfc88c, 0x1fba218aef40a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x657043e7b0194, 0x5c11b55efe9e7, 0x7737bc6a074fb, 0xeae41ce355cc, 0x6c535d13ff776 ]),
            y_minus_x: Fe([ 0x49448fac8f53e, 0x34f74c6e8356a, 0xad780607dba2, 0x7213a7eb63eb6, 0x392e3acaa8c86 ]),
            xy2d: Fe([ 0x534e93e8a35af, 0x8b10fd02c997, 0x26ac2acb81e05, 0x9d8c98ce3b79, 0x25e17fe4d50ac ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x77ff576f121a7, 0x4e5f9b0fc722b, 0x46f949b0d28c8, 0x4cde65d17ef26, 0x6bba828f89698 ]),
            y_minus_x: Fe([ 0x9bd71e04f676, 0x25ac841f2a145, 0x1a47eac823871, 0x1a8a8c36c581a, 0x255751442a9fb ]),
            xy2d: Fe([ 0x1bc6690fe3901, 0x314132f5abc5a, 0x611835132d528, 0x5f24b8eb48a57, 0x559d504f7f6b7 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x91e7f6d266fd, 0x36060ef037389, 0x18788ec1d1286, 0x287441c478eb0, 0x123ea6a3354bd ]),
            y_minus_x: Fe([ 0x38378b3eb54d5, 0x4d4aaa78f94ee, 0x4a002e875a74d, 0x10b851367b17c, 0x1ab12d5807e3 ]),
            xy2d: Fe([ 0x5189041e32d96, 0x5b062b090231, 0xc91766e7b78f, 0xaa0f55a138ec, 0x4a3961e2c918a ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x7d644f3233f1e, 0x1c69f9e02c064, 0x36ae5e5266898, 0x8fc1dad38b79, 0x68aceead9bd41 ]),
            y_minus_x: Fe([ 0x43be0f8e6bba0, 0x68fdffc614e3b, 0x4e91dab5b3be0, 0x3b1d4c9212ff0, 0x2cd6bce3fb1db ]),
            xy2d: Fe([ 0x4c90ef3d7c210, 0x496f5a0818716, 0x79cf88cc239b8, 0x2cb9c306cf8db, 0x595760d5b508f ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2cbebfd022790, 0xb8822aec1105, 0x4d1cfd226bccc, 0x515b2fa4971be, 0x2cb2c5df54515 ]),
            y_minus_x: Fe([ 0x1bfe104aa6397, 0x11494ff996c25, 0x64251623e5800, 0xd49fc5e044be, 0x709fa43edcb29 ]),
            xy2d: Fe([ 0x25d8c63fd2aca, 0x4c5cd29dffd61, 0x32ec0eb48af05, 0x18f9391f9b77c, 0x70f029ecf0c81 ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x2afaa5e10b0b9, 0x61de08355254d, 0xeb587de3c28d, 0x4f0bb9f7dbbd5, 0x44eca5a2a74bd ]),
            y_minus_x: Fe([ 0x307b32eed3e33, 0x6748ab03ce8c2, 0x57c0d9ab810bc, 0x42c64a224e98c, 0xb7d5d8a6c314 ]),
            xy2d: Fe([ 0x448327b95d543, 0x146681e3a4ba, 0x38714adc34e0c, 0x4f26f0e298e30, 0x272224512c7de ]),
        },
        GePrecomp {
            y_plus_x: Fe([ 0x3bb8a42a975fc, 0x6f2d5b46b17ef, 0x7b6a9223170e5, 0x53713fe3b7e6, 0x19735fd7f6bc2 ]),
            y_minus_x: Fe([ 0x492af49c5342e, 0x2365cdf5a0357, 0x32138a7ffbb60, 0x2a1f7d14646fe, 0x11b5df18a44cc ]),
            xy2d: Fe([ 0x390d042c84266, 0x1efe32a8fdc75, 0x6925ee7ae1238, 0x4af9281d0e832, 0xfef911191df8 ]),
        },
    ],
];
