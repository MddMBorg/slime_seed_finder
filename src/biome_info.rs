use crate::biome_layers::Biome;
// Autogenerated code
#[allow(non_upper_case_globals)]
pub mod biome_id {
pub type BiomeID = i32;
pub const bambooJungleHills: BiomeID = 169;
pub const bambooJungle: BiomeID = 168;
pub const BIOME_NUM: BiomeID = 51;
pub const frozenDeepOcean: BiomeID = 50;
// 40-49
pub const coldDeepOcean: BiomeID = 49;
pub const lukewarmDeepOcean: BiomeID = 48;
pub const warmDeepOcean: BiomeID = 47;
pub const coldOcean: BiomeID = 46;
pub const lukewarmOcean: BiomeID = 45;
pub const warmOcean: BiomeID = 44;
pub const skyIslandBarren: BiomeID = 43;
pub const skyIslandHigh: BiomeID = 42;
pub const skyIslandMedium: BiomeID = 41;
// 1.13
pub const skyIslandLow: BiomeID = 40;
// 30-39
pub const mesaPlateau: BiomeID = 39;
pub const mesaPlateau_F: BiomeID = 38;
pub const mesa: BiomeID = 37;
pub const savannaPlateau: BiomeID = 36;
pub const savanna: BiomeID = 35;
pub const extremeHillsPlus: BiomeID = 34;
pub const megaTaigaHills: BiomeID = 33;
pub const megaTaiga: BiomeID = 32;
pub const coldTaigaHills: BiomeID = 31;
pub const coldTaiga: BiomeID = 30;
// 20-29
pub const roofedForest: BiomeID = 29;
pub const birchForestHills: BiomeID = 28;
pub const birchForest: BiomeID = 27;
pub const coldBeach: BiomeID = 26;
pub const stoneBeach: BiomeID = 25;
pub const deepOcean: BiomeID = 24;
pub const jungleEdge: BiomeID = 23;
pub const jungleHills: BiomeID = 22;
pub const jungle: BiomeID = 21;
pub const extremeHillsEdge: BiomeID = 20;
// 10-19
pub const taigaHills: BiomeID = 19;
pub const forestHills: BiomeID = 18;
pub const desertHills: BiomeID = 17;
pub const beach: BiomeID = 16;
pub const mushroomIslandShore: BiomeID = 15;
pub const mushroomIsland: BiomeID = 14;
pub const iceMountains: BiomeID = 13;
pub const icePlains: BiomeID = 12;
pub const frozenRiver: BiomeID = 11;
pub const frozenOcean: BiomeID = 10;
// 0-9
pub const sky: BiomeID = 9;
pub const hell: BiomeID = 8;
pub const river: BiomeID = 7;
pub const swampland: BiomeID = 6;
pub const taiga: BiomeID = 5;
pub const forest: BiomeID = 4;
pub const extremeHills: BiomeID = 3;
pub const desert: BiomeID = 2;
pub const plains: BiomeID = 1;
pub const ocean: BiomeID = 0;
pub const none: BiomeID = -1;
pub type BiomeType = i32;
pub const BTYPE_NUM: BiomeType = 17;
pub const Mesa: BiomeType = 16;
pub const Savanna: BiomeType = 15;
pub const StoneBeach: BiomeType = 14;
pub const Jungle: BiomeType = 13;
pub const Beach: BiomeType = 12;
pub const MushroomIsland: BiomeType = 11;
pub const Snow: BiomeType = 10;
pub const Sky: BiomeType = 9;
pub const Hell: BiomeType = 8;
pub const River: BiomeType = 7;
pub const Swamp: BiomeType = 6;
pub const Taiga: BiomeType = 5;
pub const Forest: BiomeType = 4;
pub const Hills: BiomeType = 3;
pub const Desert: BiomeType = 2;
pub const Plains: BiomeType = 1;
pub const Ocean: BiomeType = 0;
pub type BiomeTempCategory = i32;
pub const Unknown: BiomeTempCategory = 5;
pub const Freezing: BiomeTempCategory = 4;
pub const Cold: BiomeTempCategory = 3;
pub const Lush: BiomeTempCategory = 2;
pub const Warm: BiomeTempCategory = 1;
pub const Oceanic: BiomeTempCategory = 0;
}

// TODO: I changed 252 to pure green to help with debugging
pub const UNKNOWN_BIOME_ID: i32 = 252;
pub static BIOME_COLORS: [[u8; 3]; 256] =
[[0, 0, 112], [141, 179, 96], [250, 148, 24], [96, 96, 96], [5, 102, 33], [11, 102, 89], [7, 249, 178], [0, 0, 255], [255, 0, 0], [128, 128, 255], [112, 112, 214], [160, 160, 255], [255, 255, 255], [160, 160, 160], [255, 0, 255], [160, 0, 255], [250, 222, 85], [210, 95, 18], [34, 85, 28], [22, 57, 51], [114, 120, 154], [83, 123, 9], [44, 66, 5], [98, 139, 23], [0, 0, 48], [162, 162, 132], [250, 240, 192], [48, 116, 68], [31, 95, 50], [64, 81, 26], [49, 85, 74], [36, 63, 54], [89, 102, 81], [69, 79, 62], [80, 112, 80], [189, 178, 95], [167, 157, 100], [217, 69, 21], [176, 151, 101], [202, 140, 101], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 172], [0, 0, 144], [32, 32, 112], [0, 0, 80], [0, 0, 64], [32, 32, 56], [64, 64, 144], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 112], [141, 179, 96], [250, 148, 24], [96, 96, 96], [5, 102, 33], [11, 102, 89], [7, 249, 178], [0, 0, 255], [255, 0, 0], [128, 128, 255], [144, 144, 160], [160, 160, 255], [140, 180, 180], [160, 160, 160], [255, 0, 255], [160, 0, 255], [250, 222, 85], [210, 95, 18], [34, 85, 28], [22, 57, 51], [114, 120, 154], [83, 123, 9], [44, 66, 5], [98, 139, 23], [0, 0, 48], [162, 162, 132], [250, 240, 192], [48, 116, 68], [31, 95, 50], [64, 81, 26], [49, 85, 74], [36, 63, 54], [89, 102, 81], [69, 79, 62], [80, 112, 80], [189, 178, 95], [167, 157, 100], [217, 69, 21], [176, 151, 101], [202, 140, 101], [118, 142, 20], [59, 71, 10], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 255, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]]
;

pub static BIOME_INFO: [Biome; 256] =
[Biome { id: 0, type_0: 0, height: -1.0, temp: 0.5, tempCat: 0 }, Biome { id: 1, type_0: 1, height: 0.10000000149011612, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 2, type_0: 2, height: 0.125, temp: 2.0, tempCat: 1 }, Biome { id: 3, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 4, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 5, type_0: 5, height: 0.20000000298023224, temp: 0.25, tempCat: 2 }, Biome { id: 6, type_0: 6, height: -0.20000000298023224, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 7, type_0: 7, height: -0.5, temp: 0.5, tempCat: 2 }, Biome { id: 8, type_0: 8, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 9, type_0: 9, height: 0.10000000149011612, temp: 0.5, tempCat: 2 }, Biome { id: 10, type_0: 0, height: -1.0, temp: 0.0, tempCat: 0 }, Biome { id: 11, type_0: 7, height: -0.5, temp: 0.0, tempCat: 3 }, Biome { id: 12, type_0: 10, height: 0.125, temp: 0.0, tempCat: 3 }, Biome { id: 13, type_0: 10, height: 0.44999998807907104, temp: 0.0, tempCat: 3 }, Biome { id: 14, type_0: 11, height: 0.20000000298023224, temp: 0.8999999761581421, tempCat: 2 }, Biome { id: 15, type_0: 11, height: 0.0, temp: 0.8999999761581421, tempCat: 2 }, Biome { id: 16, type_0: 12, height: 0.0, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 17, type_0: 2, height: 0.44999998807907104, temp: 2.0, tempCat: 1 }, Biome { id: 18, type_0: 4, height: 0.44999998807907104, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 19, type_0: 5, height: 0.44999998807907104, temp: 0.25, tempCat: 2 }, Biome { id: 20, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 21, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 22, type_0: 13, height: 0.44999998807907104, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 23, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 24, type_0: 0, height: -1.7999999523162842, temp: 0.5, tempCat: 0 }, Biome { id: 25, type_0: 14, height: 0.10000000149011612, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 26, type_0: 12, height: 0.0, temp: 0.05000000074505806, tempCat: 3 }, Biome { id: 27, type_0: 4, height: 0.10000000149011612, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 28, type_0: 4, height: 0.44999998807907104, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 29, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 30, type_0: 5, height: 0.20000000298023224, temp: -0.5, tempCat: 3 }, Biome { id: 31, type_0: 5, height: 0.44999998807907104, temp: -0.5, tempCat: 3 }, Biome { id: 32, type_0: 5, height: 0.20000000298023224, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 33, type_0: 5, height: 0.44999998807907104, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 34, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 35, type_0: 15, height: 0.125, temp: 1.2000000476837158, tempCat: 1 }, Biome { id: 36, type_0: 15, height: 1.5, temp: 1.0, tempCat: 1 }, Biome { id: 37, type_0: 16, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 38, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 39, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 40, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 41, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 42, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 43, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 44, type_0: 0, height: -1.0, temp: 0.0, tempCat: 0 }, Biome { id: 45, type_0: 0, height: -1.0, temp: 0.0, tempCat: 0 }, Biome { id: 46, type_0: 0, height: -1.0, temp: 0.0, tempCat: 0 }, Biome { id: 47, type_0: 0, height: -1.8, temp: 0.0, tempCat: 0 }, Biome { id: 48, type_0: 0, height: -1.8, temp: 0.0, tempCat: 0 }, Biome { id: 49, type_0: 0, height: -1.8, temp: 0.0, tempCat: 0 }, Biome { id: 50, type_0: 0, height: -1.8, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 129, type_0: 1, height: 0.10000000149011612, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 130, type_0: 2, height: 0.125, temp: 2.0, tempCat: 1 }, Biome { id: 131, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 132, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 133, type_0: 5, height: 0.20000000298023224, temp: 0.25, tempCat: 2 }, Biome { id: 134, type_0: 6, height: -0.20000000298023224, temp: 0.800000011920929, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 140, type_0: 10, height: 0.125, temp: 0.0, tempCat: 3 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 149, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 151, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 155, type_0: 4, height: 0.10000000149011612, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 156, type_0: 4, height: 0.44999998807907104, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 157, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 158, type_0: 5, height: 0.20000000298023224, temp: -0.5, tempCat: 3 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 160, type_0: 5, height: 0.20000000298023224, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 161, type_0: 5, height: 0.44999998807907104, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 162, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 163, type_0: 15, height: 0.125, temp: 1.2000000476837158, tempCat: 1 }, Biome { id: 164, type_0: 15, height: 1.5, temp: 1.0, tempCat: 1 }, Biome { id: 165, type_0: 16, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 166, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 167, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 168, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 169, type_0: 13, height: 0.44999998807907104, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }]
;
