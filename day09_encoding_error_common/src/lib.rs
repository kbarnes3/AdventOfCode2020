pub const SAMPLE_DATA_PREAMBLE: usize = 5;

pub const SAMPLE_DATA: [u64; 20] = [
    35,
    20,
    15,
    25,
    47,
    40,
    62,
    55,
    65,
    95,
    102,
    117,
    150,
    182,
    127,
    219,
    299,
    277,
    309,
    576,
];

pub const REAL_DATA_PREAMBLE: usize = 25;

pub const REAL_DATA: [u64; 1000] = [
    8,
    11,
    27,
    48,
    5,
    19,
    18,
    36,
    3,
    45,
    22,
    37,
    46,
    4,
    31,
    42,
    39,
    30,
    41,
    14,
    40,
    29,
    43,
    25,
    6,
    7,
    28,
    8,
    52,
    9,
    49,
    13,
    20,
    34,
    10,
    11,
    12,
    17,
    15,
    18,
    53,
    21,
    16,
    26,
    19,
    14,
    54,
    39,
    22,
    23,
    24,
    25,
    27,
    28,
    29,
    80,
    33,
    35,
    30,
    31,
    72,
    32,
    34,
    36,
    37,
    38,
    48,
    60,
    47,
    51,
    41,
    45,
    49,
    67,
    50,
    64,
    62,
    77,
    57,
    65,
    66,
    68,
    71,
    107,
    108,
    69,
    70,
    119,
    73,
    94,
    79,
    117,
    86,
    88,
    90,
    133,
    202,
    99,
    116,
    141,
    122,
    148,
    218,
    223,
    131,
    134,
    137,
    139,
    258,
    142,
    155,
    143,
    152,
    161,
    263,
    259,
    219,
    174,
    178,
    189,
    420,
    215,
    221,
    238,
    261,
    253,
    393,
    265,
    440,
    270,
    271,
    311,
    482,
    285,
    294,
    443,
    295,
    313,
    335,
    389,
    367,
    352,
    363,
    663,
    810,
    491,
    526,
    459,
    572,
    712,
    629,
    535,
    565,
    541,
    660,
    556,
    832,
    580,
    579,
    589,
    648,
    702,
    1351,
    687,
    715,
    1000,
    811,
    952,
    1907,
    950,
    1239,
    1031,
    1145,
    1161,
    1076,
    1258,
    1204,
    1515,
    1121,
    1135,
    1530,
    1159,
    1168,
    1276,
    1237,
    1335,
    1389,
    1402,
    1498,
    1526,
    1761,
    1763,
    1902,
    1981,
    2026,
    2917,
    2363,
    2289,
    2197,
    2211,
    2833,
    2724,
    3546,
    2327,
    2294,
    2396,
    3448,
    2557,
    4275,
    3096,
    2737,
    2791,
    3024,
    3259,
    3960,
    3524,
    4159,
    3883,
    4007,
    5811,
    4408,
    4486,
    4500,
    4491,
    4505,
    8434,
    5118,
    4621,
    4690,
    4851,
    5844,
    5815,
    5348,
    6120,
    5528,
    7683,
    6283,
    7750,
    8697,
    7407,
    7531,
    7890,
    9029,
    10971,
    8894,
    10876,
    8991,
    9112,
    8996,
    10349,
    9472,
    9311,
    9541,
    10038,
    10199,
    11468,
    11163,
    11811,
    18423,
    14519,
    13690,
    18089,
    20474,
    17756,
    16919,
    15421,
    20057,
    17885,
    18435,
    17890,
    17987,
    18103,
    18653,
    18537,
    24557,
    29914,
    30609,
    19579,
    25459,
    21362,
    22631,
    45516,
    25501,
    31446,
    28209,
    29111,
    33856,
    32340,
    38116,
    33306,
    33311,
    35877,
    36325,
    66325,
    35993,
    36756,
    43562,
    43094,
    50877,
    53668,
    65302,
    40941,
    46821,
    43993,
    54673,
    86119,
    73725,
    69299,
    61451,
    57320,
    62417,
    69096,
    88993,
    71422,
    105176,
    69188,
    79087,
    72318,
    79555,
    80749,
    105550,
    131605,
    84035,
    126508,
    116120,
    119737,
    84934,
    106410,
    133839,
    205113,
    118771,
    123868,
    126416,
    141972,
    141504,
    143740,
    138284,
    140610,
    151873,
    159836,
    148275,
    151405,
    188438,
    164489,
    164784,
    224645,
    283704,
    283555,
    409971,
    191344,
    246153,
    203705,
    278607,
    311709,
    262511,
    242639,
    250284,
    264700,
    316189,
    500147,
    372920,
    278894,
    288885,
    342749,
    299680,
    453989,
    355833,
    329273,
    356128,
    368489,
    395049,
    433983,
    437497,
    618639,
    441628,
    446344,
    564380,
    579557,
    492923,
    539169,
    615869,
    711238,
    543594,
    762533,
    567779,
    578574,
    588565,
    642429,
    962828,
    628953,
    790111,
    685106,
    935685,
    1369668,
    763538,
    1122603,
    871480,
    930420,
    1372048,
    887972,
    1958233,
    1057303,
    1032092,
    1036517,
    1082763,
    1111373,
    1966937,
    2045591,
    1357890,
    1146353,
    2179906,
    1217518,
    1271382,
    1850800,
    1392491,
    1475217,
    2034920,
    1699223,
    2629272,
    1635018,
    2586590,
    2735740,
    3015536,
    3669938,
    1920064,
    2119280,
    2068609,
    3091714,
    2147890,
    2194136,
    2257726,
    2363871,
    2417735,
    5273262,
    2488900,
    3395281,
    3885720,
    2663873,
    4714759,
    2867708,
    3110235,
    6634823,
    3334241,
    3555082,
    4557509,
    5509449,
    5755587,
    5178844,
    4114200,
    3988673,
    4326335,
    5061844,
    4342026,
    4405616,
    6201949,
    5972817,
    4852771,
    4906635,
    5152773,
    6043982,
    8240876,
    6665317,
    5531581,
    6422790,
    9744260,
    6444476,
    8440535,
    7448441,
    9258387,
    8102873,
    8315008,
    11222826,
    8330699,
    12510285,
    9050517,
    8668361,
    8747642,
    9194797,
    9558389,
    10005544,
    9759406,
    10950617,
    10059408,
    10684354,
    18807050,
    11954371,
    11976057,
    15981179,
    16503884,
    14759484,
    14547349,
    15551314,
    15763449,
    17942439,
    16417881,
    16645707,
    16999060,
    24298956,
    19764950,
    17416003,
    18306031,
    18753186,
    18954203,
    20509006,
    29437540,
    21735463,
    20743762,
    26665533,
    25443838,
    40716837,
    23930428,
    26735541,
    29306833,
    48060019,
    30098663,
    34415063,
    39677902,
    33063588,
    33416941,
    37059217,
    33644767,
    35305091,
    46853543,
    35722034,
    36370206,
    37260234,
    37707389,
    39463209,
    42244469,
    54881378,
    42479225,
    44674190,
    60082474,
    49374266,
    50665969,
    53237261,
    56042374,
    68722032,
    63515604,
    63162251,
    66480529,
    76723443,
    70677175,
    69138975,
    68949858,
    69366801,
    71027125,
    86918659,
    72092240,
    82381579,
    74967623,
    77170598,
    107836441,
    98521599,
    102561699,
    95340159,
    123244725,
    114181573,
    100040235,
    103903230,
    123914436,
    126677855,
    139976983,
    197704980,
    164706960,
    139816150,
    167432399,
    141231215,
    213596514,
    143119365,
    140393926,
    145994748,
    147059863,
    152138221,
    172510757,
    238915525,
    216986748,
    193861758,
    195380394,
    268610190,
    199243389,
    246034983,
    230581085,
    226718090,
    227817666,
    250592291,
    346303252,
    279793133,
    291954371,
    280210076,
    465633615,
    281625141,
    315630122,
    283513291,
    409004116,
    345999979,
    341375142,
    299198084,
    367891151,
    485596938,
    477375049,
    514645173,
    422098484,
    394623783,
    425961479,
    458398751,
    454535756,
    992020222,
    477310381,
    591967433,
    530385424,
    560003209,
    923649289,
    573579512,
    565138432,
    580823225,
    774028873,
    760823672,
    582711375,
    721296568,
    640573226,
    667089235,
    693821867,
    907695422,
    1506360664,
    816722267,
    871934164,
    820585262,
    1008672854,
    880497235,
    1023537183,
    756008079,
    1334403184,
    1007695805,
    1090388633,
    1095523856,
    1125141641,
    1138717944,
    1145961657,
    1223284601,
    2108432057,
    1249800610,
    1276533242,
    1307662461,
    1334395093,
    2090411263,
    1360911102,
    1449829946,
    1572730346,
    2181496364,
    1828281067,
    1576593341,
    1966546919,
    1636505314,
    1763703884,
    1779545262,
    1846396712,
    2098084438,
    2132837446,
    2271103298,
    3404874408,
    2263859585,
    3401011413,
    2369246258,
    3540392827,
    4477678315,
    2849263588,
    2584195703,
    3274209380,
    2937504443,
    4977604754,
    3730157360,
    3022560292,
    3416050576,
    3213098655,
    3340297225,
    3356138603,
    3400209198,
    3482902026,
    3610100596,
    3912382708,
    4369187736,
    5638477265,
    5484201953,
    5113123173,
    7376982758,
    7769396934,
    6477897270,
    4953441961,
    5433459291,
    5606755995,
    5960064735,
    6740506423,
    6211713823,
    6150603098,
    7391748028,
    8824499178,
    6235658947,
    6966239199,
    7395284734,
    6696435828,
    6839040629,
    6883111224,
    9025505881,
    7522483304,
    8281570444,
    9322629697,
    11104045059,
    10066565134,
    10386901252,
    11040215286,
    15800526967,
    10560197956,
    13201898146,
    11393524026,
    15804053748,
    12110667833,
    12362316921,
    13033714322,
    25126683445,
    40930737193,
    12932094775,
    13074699576,
    13535476457,
    19916825546,
    14405594528,
    14361523933,
    27436223509,
    21483468590,
    22643094377,
    17604200141,
    19389194831,
    20453466386,
    20626763090,
    21953721982,
    21600413242,
    25564215067,
    22922514877,
    23504191859,
    31139676598,
    24472984754,
    26610176033,
    25294411696,
    26006794351,
    26467571232,
    27293618708,
    33988942843,
    27480294104,
    46682813539,
    28767118461,
    31965724074,
    33750718764,
    36993394972,
    40015957921,
    38230963231,
    38057666527,
    46190978157,
    41080229476,
    42227176332,
    43554135224,
    50479779105,
    61469236947,
    52474365583,
    47977176613,
    49767396450,
    51766603462,
    77773397813,
    51301206047,
    53300413059,
    111949016052,
    54773912812,
    68959119046,
    94701541915,
    60732842535,
    90495737026,
    65716442838,
    98790509062,
    75051061499,
    89783354371,
    76288629758,
    79137896003,
    84634364700,
    83307405808,
    85781311556,
    155503151276,
    112034048582,
    121433484629,
    133074802258,
    189286246088,
    101068602497,
    183424873762,
    126352267546,
    104601619106,
    120490355650,
    115506755347,
    123733031858,
    126449285373,
    144040248343,
    191795385105,
    241923840279,
    180206498500,
    159685426199,
    155426525761,
    344789397364,
    159596035566,
    162445301811,
    167941770508,
    169088717364,
    220108374453,
    205670221603,
    298931629109,
    216575357844,
    221558958147,
    235997110997,
    319281461765,
    225091974756,
    356487466647,
    228334650964,
    422245579447,
    239239787205,
    250182317231,
    270489533716,
    299466774104,
    315022561327,
    384004259958,
    315111951960,
    317871827572,
    322041337377,
    598398403213,
    569421162825,
    330387072319,
    535130935780,
    468020346473,
    425778596056,
    427229179750,
    441667332600,
    438134315991,
    464331761961,
    475236898202,
    742341131710,
    453426625720,
    498824184680,
    467574438169,
    580569389550,
    489422104436,
    811463441813,
    1002705373949,
    614489335431,
    645499024279,
    632983779532,
    757616252069,
    820865522057,
    652428409696,
    863912912047,
    890110358017,
    910956461869,
    865363495741,
    853007775806,
    1108220677734,
    952250810400,
    879801648591,
    1082063773600,
    917758387681,
    1105855035416,
    921001063889,
    1113313520111,
    1113073462448,
    1353335016483,
    1433577165356,
    1103911439867,
    1266917745127,
    1247473114963,
    1285412189228,
    1516341321743,
    1410044661765,
    1532230058287,
    1832052458991,
    1505436185502,
    2132281240868,
    1718371271547,
    1732809424397,
    1745165144332,
    1770766163487,
    2360786635074,
    1797560036272,
    1800802712480,
    2185975213467,
    2030831850129,
    2024912503756,
    2331045725654,
    2466408478931,
    2360546577411,
    2351384554830,
    3234379877836,
    2992638259295,
    2980282539360,
    2532885304191,
    2695456850993,
    3277395202619,
    2915480847267,
    3223807457049,
    3238245609899,
    3515931307819,
    3463536415879,
    3918784637864,
    3530369460669,
    3542725180604,
    3568326199759,
    3598362748752,
    3822472540028,
    7352842000697,
    5567637684360,
    4726288701122,
    4355958229410,
    5972920798655,
    7084257507578,
    4711931132241,
    5448366151458,
    5228342155184,
    5513167843551,
    5610937698260,
    5756692761240,
    7060718149927,
    7111051380363,
    6139288304316,
    8175467548120,
    9211166408554,
    14439508563738,
    10883190689955,
    9209300447012,
    7073094641273,
    7141087929356,
    8796668354943,
    7420835288780,
    8178430769438,
    10337226399382,
    11776793518190,
    12301436796457,
    9067889361651,
    10160297283699,
    10468623893481,
    9940273287425,
    10676708306642,
    13932160309360,
    11124105541811,
    11367630459500,
    11895981065556,
    13250339684679,
    15596302836900,
    15251525410711,
    14214182570629,
    19265292248424,
    25299790768860,
    15319518698794,
    18647054662919,
    14493929930053,
    14561923218136,
    15599266058218,
    20805850292863,
    23190612972104,
    20844682879841,
    19744597668293,
    26110163636185,
    30161189276354,
    20100570571124,
    20408897180906,
    28426090239413,
    22491736001311,
    23020086607367,
    24617970144179,
    26389910995609,
    25146320750235,
    29745455340764,
    34584810947218,
    29055853148189,
    28708112500682,
    29813448628847,
    50154352521670,
    34306520886429,
    49464750329095,
    60416684522614,
    35699836629342,
    35343863726511,
    39845168239417,
    40153494849199,
    51446176846780,
    70284647576560,
    64330266287982,
    59558903969611,
    45246891321359,
    46798808176515,
    45511822608678,
    47109706145490,
    65299815599434,
    85752697733209,
    95666175130348,
    94143714916829,
    57763965648871,
    114764565928529,
    70006357515771,
];
