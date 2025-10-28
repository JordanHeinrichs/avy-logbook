use serde::{Deserialize, Serialize};
use sqlx::error::BoxDynError;
use sqlx::types::Type;
use sqlx::Database;
use sqlx::{encode::IsNull, Decode, Encode, FromRow, Sqlite};
use std::ops::{Deref, DerefMut};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum DangerRating {
    Low,
    Moderate,
    Considerable,
    High,
    Extreme,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum MacroTrend {
    Decreasing,
    Steady,
    MeltFreeze,
    Increasing,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum Confidence {
    High,
    Moderate,
    Low,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum ProblemType {
    LooseDry,
    LooseWet,
    WetSlab,
    Cornices,
    WindSlabs,
    StormSlabs,
    PersistentSlabs,
    DeepPersistentSlabs,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum Precipitation {
    Nothing,
    S1,     // Light snowfall <= 1 cm/hr
    RL,     // Light Rain
    S2,     // Moderate snowfall 2 cm/hr
    R,      // Rain
    S3Plus, // Heavy snowfall >= 3 cm/hr
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum Accumulation {
    Zero,
    LessThan20,
    From20To40,
    MoreThan40,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum WindSpeed {
    Calm,
    Light,
    Moderate,
    Strong,
    Extreme,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
pub enum WindDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum SolarRadiation {
    None,
    Weak,
    Moderate,
    Strong,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum AvalancheSize {
    None,
    LessThan1,
    From1_5To2,
    MoreThan2_5,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum AvalancheTrigger {
    Heavy,
    Moderate,
    Light,
    Natural,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum InstabilityObservation {
    None,
    Drum,
    Crack,
    Whumpf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum TestResult {
    None,
    Failure,
    PopDrop,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub enum AreaToAvoid {
    TerrainTraps,
    StartZones,
    AvyPaths,
    RunoutZones,
    Slope30To35,
    SlopeGreaterThan35,
    ConvexUnsupported,
    LeeLoaded,
    Sunny,
    SlopeSizeLarge,
    SlopeSizeMedium,
    SlopeSizeSmall,
    OverheadHazard,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, sqlx::Type, TS)]
#[sqlx(type_name = "TEXT")]
#[ts(export)]
pub enum Elevation {
    #[serde(rename = "ALP")]
    Alp,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "BTL")]
    Btl,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AreasToAvoid(pub Vec<AreaToAvoid>);

impl Deref for AreasToAvoid {
    type Target = Vec<AreaToAvoid>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AreasToAvoid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Type<Sqlite> for AreasToAvoid {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}
impl<'r> Decode<'r, Sqlite> for AreasToAvoid {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let str_value = <&str as Decode<Sqlite>>::decode(value)?;
        let vec_value: Vec<AreaToAvoid> = serde_json::from_str(str_value)?;
        Ok(AreasToAvoid(vec_value))
    }
}
impl<'q> Encode<'q, Sqlite> for AreasToAvoid {
    fn encode_by_ref(
        &self,
        args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>,
    ) -> Result<IsNull, BoxDynError> {
        let json_string = serde_json::to_string(&self.0).unwrap_or_else(|_| "[]".to_string());
        args.push(sqlx::sqlite::SqliteArgumentValue::Text(json_string.into()));
        Ok(IsNull::No)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Trip {
    #[ts(type = "number")]
    pub id: i64,
    pub name: String,
    pub trip_date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AvalancheForecast {
    #[ts(type = "number")]
    pub id: i64,
    #[ts(type = "number")]
    pub trip_id: i64,
    pub forecast_alp: Option<DangerRating>,
    pub forecast_tl: Option<DangerRating>,
    pub forecast_btl: Option<DangerRating>,
    pub macro_trends: Option<MacroTrend>,
    pub confidence: Option<Confidence>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AvalancheProblem {
    #[ts(type = "number")]
    pub id: i64,
    #[ts(type = "number")]
    pub forecast_id: i64,
    pub elevation: Elevation,
    pub problem_type: ProblemType,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct TripPlanning {
    #[ts(type = "number")]
    pub id: i64,
    #[ts(type = "number")]
    pub trip_id: i64,
    pub areas_to_avoid: Option<AreasToAvoid>,
    pub plan_left_with_someone: bool,
    pub decision_points_considered: bool,
    pub decision_points_comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, TS)]
#[serde(rename_all = "camelCase")]
pub struct FieldObservation {
    #[ts(type = "number")]
    pub id: i64,
    #[ts(type = "number")]
    pub trip_id: i64,
    pub observation_time: String,

    // Weather
    pub precipitation: Option<Precipitation>,
    pub accumulation: Option<Accumulation>,
    pub wind_speed: Option<WindSpeed>,
    pub wind_direction: Option<WindDirection>,
    pub solar_radiation: Option<SolarRadiation>,

    // Avalanche Activity
    pub avy_activity_size: Option<AvalancheSize>,
    pub avy_activity_trigger: Option<AvalancheTrigger>,
    pub avy_activity_characteristic: Option<ProblemType>,

    // Signs of Instability
    pub instability_see_feel: Option<InstabilityObservation>,
    pub instability_ct: Option<TestResult>,
    pub instability_ect: Option<TestResult>,

    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct FullTripDetails {
    #[serde(flatten)]
    pub trip: Trip,

    pub forecast: Option<AvalancheForecast>,
    pub planning: Option<TripPlanning>,

    pub problems: Vec<AvalancheProblem>,
    pub observations: Vec<FieldObservation>,
}
