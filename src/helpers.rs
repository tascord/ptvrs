use chrono::NaiveDateTime;
use itertools::Itertools;
use serde::{de::SeqAccess, ser, Deserialize, Deserializer, Serialize};

use crate::DisruptionModes;

pub fn clean(s: String) -> String {
    let mut s = s;
    s = s.trim().to_string();
    s = s.trim_start_matches('"').to_string();
    s = s.trim_end_matches('"').to_string();
    s
}

pub fn to_query<T: Serialize>(s: T) -> String {
    serde_json::to_value(s)
        .unwrap()
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            // If v is an array, define k={v[0]}&k={v[1]}&...
            if v.is_array() {
                v.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| {
                        format!(
                            "{}={}",
                            k,
                            url_escape::encode_query(&clean(v.to_string())).into_owned()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("&")
            } else {
                format!("{}={}", k, clean(v.to_string()))
            }
        })
        .collect::<Vec<String>>()
        .join("&")
}

pub fn de_iso_8601<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
        .map_err(|e| serde::de::Error::custom(format!("Error deser iso_8601 '{s}': {e:?}")))
}

pub fn ser_iso_8601<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date {
        Some(date) => serializer.serialize_str(&date.format("%Y-%m-%dT%H:%M:%S").to_string()),
        None => serializer.serialize_none(),
    }
}

/// 24 hour clock format (HH:MM:SS) AEDT/AEST
pub fn de_service_time<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                Ok(Some(
                    NaiveDateTime::parse_from_str(&s, "%H:%M:%S").map_err(|e| {
                        serde::de::Error::custom(format!("Error deser service_time '{s}': {e:?}"))
                    })?,
                ))
            }
        }
        None => Ok(None),
    }
}

// yyyy-MM-dd HH:mm
pub fn ser_touch_utc<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date {
        Some(date) => serializer.serialize_str(&date.format("%Y-%m-%d %H:%M").to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn de_rfc3339<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3fZ")
        .map_err(|e| serde::de::Error::custom(format!("Error deser rfc3339 '{s}': {e:?}")))
}

pub fn opt_de_rfc3339<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3fZ").map_err(|e| {
                serde::de::Error::custom(format!("Error deser rfc3339 '{s}': {e:?}"))
            })?,
        )),
        None => Ok(None),
    }
}

pub fn deserialize_path<'de, D>(deserializer: D) -> Result<Vec<Vec<(f64, f64)>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct PathVisitor;
    impl<'de> serde::de::Visitor<'de> for PathVisitor {
        type Value = Vec<Vec<(f64, f64)>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a list of lists of tuples of f64")
        }
        // primarily deserializing [
        //  "-37.8683203000288, 145.079655599963 -37.8655618999753, 145.080159200029 -37.8643671999759, 145.080394800045 -37.8637750999515, 145.080557999986 -37.8635422999975, 145.080654900021 -37.8582984000176, 145.082927900024 -37.8579842999768, 145.083004200024 -37.8572559000092, 145.083102600027 -37.8562641000235, 145.083071499962 -37.8556125999982, 145.082906500037 -37.8554588999607, 145.082876400034 -37.8545596999964, 145.082445099949 -37.8437658999819, 145.075417500015 -37.8425731999696, 145.074675700034 -37.8364900000393, 145.070811399995 -37.8348502000055, 145.069729200021 -37.8342419999945, 145.069460999968 -37.8338166999764, 145.069358400021 -37.8330147999865, 145.069368000002 -37.832817199959, 145.069407200039 -37.832754099991, 145.069408900034 -37.8310293000002, 145.069737900034 -37.8283767000385, 145.070102400024 -37.8280790999898, 145.070087399984 -37.8277532999815, 145.070005000044 -37.8271535999531, 145.069702499947 -37.8267788000005, 145.06938279996 -37.8265577999701, 145.069093199963 -37.8263718999837, 145.068745799971 -37.8261712000388, 145.068046599958 -37.8261099999648, 145.067616499972 -37.8261036999724, 145.06723039997 -37.8262540999595, 145.065965300042 -37.8262642999546, 145.0654879 -37.8262822000221, 145.06493069998 -37.8263466999918, 145.06337249997 -37.8264930000373, 145.061857600011 -37.8265761000108, 145.058696899958",
        //  "-37.8265761000108, 145.058696899958 -37.8265754000173, 145.058651500023 -37.8265763000059, 145.057617500001 -37.826581799995, 145.057401500045 -37.8265383999863, 145.056959599996 -37.8264497000363, 145.056496099966 -37.8262976999717, 145.056022899954 -37.8259514000122, 145.055247999985 -37.8256736999505, 145.054800899955 -37.8244995000169, 145.053547899972 -37.824084899963, 145.053002099991 -37.8238341999955, 145.052554200003 -37.8236089999777, 145.052014799968 -37.8233684000126, 145.051089500046 -37.8230483000229, 145.049177900008 -37.8229320999804, 145.048692500033 -37.8225329000186, 145.04689660003 -37.8223983000171, 145.045843600042 -37.8223874000308, 145.045730300034 -37.8223666999572, 145.045571800029 -37.8223513000041, 145.045185899959 -37.8223313999499, 145.045072800009 -37.8221798999673, 145.0424638 -37.821867500024, 145.039938600033 -37.8217714999782, 145.039043700026 -37.8213715000086, 145.035589300009 -37.8213602000302, 145.035453200002 -37.8213348000065, 145.033931600034 -37.8212586999663, 145.032070400043 -37.8210688000277, 145.030416799984 -37.8208471000355, 145.029013999973 -37.8206689999712, 145.027530499997 -37.8206078000258, 145.026566500027 -37.8206462000339, 145.026167899985 -37.8208369999824, 145.025185800001 -37.8208875999509, 145.024979899976 -37.8211090000076, 145.024212899959 -37.8212744999776, 145.023867599988 -37.8215449999596, 145.023337799954 -37.8218259999648, 145.022898600037 -37.8221793999933, 145.022480200043 -37.8226675999865, 145.02203539995 -37.8242175999786, 145.019892300033 -37.8247437999656, 145.019026099987 -37.8255391999616, 145.017561899967 -37.8257320000204, 145.017238700006 -37.8258198000133, 145.017099999966 -37.8259512999542, 145.016880599982 -37.8260128000115, 145.016788099958 -37.826949699979, 145.015160999955 -37.8272185999868, 145.014540299957 -37.8274428000346, 145.013943500001 -37.8275291999788, 145.013725299965 -37.8276387000278, 145.013267899969 -37.8278159999823, 145.012558699971 -37.8279087000195, 145.011647299991 -37.8279017999988, 145.010704400053 -37.82756019996, 145.007555100033 -37.8274864999539, 145.006932200036 -37.8261322000209, 144.994573400054 -37.8260058000082, 144.994031500047 -37.8256500000113, 144.993245800019 -37.825529800031, 144.993067300004 -37.8252169000015, 144.992689499967 -37.8250692999908, 144.992489000002 -37.8249492999501, 144.992321799946 -37.8247286999589, 144.992066499959 -37.8244776999975, 144.991618799972 -37.8245284999882, 144.991424300051 -37.8243490000252, 144.990940700042 -37.8241955999787, 144.990399499978 -37.8241011999923, 144.99015210001 -37.8236761000357, 144.989004800006 -37.8228364999981, 144.985755700012 -37.822291599961, 144.984452700014 -37.8218150999527, 144.983465899969 -37.8201561999773, 144.98079590002 -37.8192841999558, 144.979342799971 -37.8184652000271, 144.977831600038 -37.8178993000183, 144.976892799959 -37.8173584000181, 144.975828300025 -37.8170090999653, 144.974906399995 -37.8168934000328, 144.974466499945 -37.8168523000383, 144.974172200015 -37.8167973000098, 144.973594399995 -37.8167826000027, 144.973265400031 -37.8167958999745, 144.972992400029 -37.8168180000008, 144.972707799979 -37.8169317999936, 144.97198899997 -37.8173818000015, 144.970386299946 -37.8177335000255, 144.968831700008 -37.8180060999924, 144.96791540002 -37.8180839999503, 144.967731500016 -37.8183051000073, 144.966964299956",
        //  "-37.8201561999773, 144.98079590002 -37.8177107999522, 144.97693200003 -37.8171955999625, 144.97632120001 -37.8166556000173, 144.975836200011 -37.8162349999771, 144.975484099993 -37.8159246000112, 144.975254099958 -37.8156684999692, 144.975033899985 -37.8155869000169, 144.975002000052 -37.8152870999753, 144.974862500026 -37.8150237000143, 144.974744800009 -37.8087737999705, 144.97184869999 -37.8086363000065, 144.971716200048 -37.8085259000299, 144.971582900021 -37.8083964000279, 144.971393299951 -37.8082289999986, 144.971091199971 -37.808172399957, 144.970945099968 -37.8080764999841, 144.970606999995 -37.808025400012, 144.970256300053 -37.8080015000372, 144.969916199985 -37.8079953999606, 144.96956419998 -37.808034999982, 144.969245100018 -37.8080848999907, 144.969005199993 -37.8099386999852, 144.962593500014 -37.8103137999581, 144.961356399982 -37.8117517000372, 144.956443799957 -37.8133308999864, 144.950868300016 -37.8134615000047, 144.950603399969 -37.8135931000251, 144.950395299996 -37.8138054999956, 144.950173599965 -37.81398329996, 144.950032400054 -37.81418889995, 144.949935799977 -37.8145199999943, 144.949801699966 -37.8147624000128, 144.949749500022 -37.8149150999922, 144.949722600005 -37.8151223000106, 144.949716899987 -37.8153748999829, 144.949732599976 -37.8154472999846, 144.949753299957 -37.8156823000166, 144.949792299978 -37.8161265999766, 144.949950400014 -37.8163266000045, 144.950047100001 -37.8172413000239, 144.950828299953 -37.8174253999886, 144.951050400047 -37.8194719000155, 144.952652399974 -37.820146400037, 144.953088200015 -37.8203663000234, 144.953297999947 -37.820983800037, 144.954087499969 -37.8211138999842, 144.95431110001 -37.8212360999886, 144.954603099955 -37.8213497000187, 144.954918100029 -37.8214091000209, 144.955223199949 -37.8214777999948, 144.955550700002 -37.8214793999859, 144.955641600043 -37.8214930000136, 144.95590249995 -37.8214808999926, 144.956243599998 -37.821366399953, 144.956917099953 -37.8210132000064, 144.957858399986 -37.8199780000159, 144.960522499975 -37.8198747999727, 144.960809400014 -37.8197300000066, 144.961290500009 -37.8196731999909, 144.961655599964 -37.819358000036, 144.962709400051 -37.8190125000142, 144.964093500022 -37.8188720999518, 144.964835699955 -37.8183051000073, 144.966964299956"
        //]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut paths = Vec::new();
            while let Some(path) = seq.next_element::<String>()? {
                // take a
                // "-37.8201561999773, 144.98079590002 -37.8177107999522, 144.97693200003 -37.8171955999625, 144.97632120001 -37.8166556000173, 144.975836200011 -37.8162349999771, 144.975484099993 -37.8159246000112, 144.975254099958 -37.8156684999692, 144.975033899985 -37.8155869000169, 144.975002000052 -37.8152870999753, 144.974862500026 -37.8150237000143, 144.974744800009 -37.8087737999705, 144.97184869999 -37.8086363000065, 144.971716200048 -37.8085259000299, 144.971582900021 -37.8083964000279, 144.971393299951 -37.8082289999986, 144.971091199971 -37.808172399957, 144.970945099968 -37.8080764999841, 144.970606999995 -37.808025400012, 144.970256300053 -37.8080015000372, 144.969916199985 -37.8079953999606, 144.96956419998 -37.808034999982, 144.969245100018 -37.8080848999907, 144.969005199993 -37.8099386999852, 144.962593500014 -37.8103137999581, 144.961356399982 -37.8117517000372, 144.956443799957 -37.8133308999864, 144.950868300016 -37.8134615000047, 144.950603399969 -37.8135931000251, 144.950395299996 -37.8138054999956, 144.950173599965 -37.81398329996, 144.950032400054 -37.81418889995, 144.949935799977 -37.8145199999943, 144.949801699966 -37.8147624000128, 144.949749500022 -37.8149150999922, 144.949722600005 -37.8151223000106, 144.949716899987 -37.8153748999829, 144.949732599976 -37.8154472999846, 144.949753299957 -37.8156823000166, 144.949792299978 -37.8161265999766, 144.949950400014 -37.8163266000045, 144.950047100001 -37.8172413000239, 144.950828299953 -37.8174253999886, 144.951050400047 -37.8194719000155, 144.952652399974 -37.820146400037, 144.953088200015 -37.8203663000234, 144.953297999947 -37.820983800037, 144.954087499969 -37.8211138999842, 144.95431110001 -37.8212360999886, 144.954603099955 -37.8213497000187, 144.954918100029 -37.8214091000209, 144.955223199949 -37.8214777999948, 144.955550700002 -37.8214793999859, 144.955641600043 -37.8214930000136, 144.95590249995 -37.8214808999926, 144.956243599998 -37.821366399953, 144.956917099953 -37.8210132000064, 144.957858399986 -37.8199780000159, 144.960522499975 -37.8198747999727, 144.960809400014 -37.8197300000066, 144.961290500009 -37.8196731999909, 144.961655599964 -37.819358000036, 144.962709400051 -37.8190125000142, 144.964093500022 -37.8188720999518, 144.964835699955 -37.8183051000073, 144.966964299956"
                // and convert it into a Vec<(f64, f64)> like
                // [(-37.8201561999773, 144.98079590002)]
                paths.push(
                    path.split(' ')
                        .chunks(2)
                        .into_iter()
                        .map(|mut chunk| {
                            let lat = chunk
                                .next()
                                .and_then(|x| x.split(',').next())
                                .ok_or_else(|| serde::de::Error::missing_field("latitude"))?
                                .parse::<f64>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!("could not parse f64: {}", e))
                                })?;
                            let lon = chunk
                                .next()
                                .and_then(|x| x.split(',').next())
                                .ok_or_else(|| serde::de::Error::missing_field("longitude"))?
                                .parse::<f64>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!("could not parse f64: {}", e))
                                })?;
                            Ok((lat, lon))
                        })
                        .collect::<Result<Vec<_>, A::Error>>()?,
                )
            }
            Ok(paths)
        }
    }
    deserializer.deserialize_any(PathVisitor)
}

pub fn ser_disruption_query<S>(
    disruption: &Option<Vec<DisruptionModes>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    match disruption {
        Some(disruption) => serializer.collect_seq(disruption.iter().map(|d| d.as_number())),
        None => serializer.serialize_none(),
    }
}

pub fn de_string_as_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = i32;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing an integer")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse::<i32>().map_err(serde::de::Error::custom)
        }
    }
    deserializer.deserialize_i32(Visitor)
}
