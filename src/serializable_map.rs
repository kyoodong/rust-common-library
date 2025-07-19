use alloc::collections::btree_map::Iter;
use core::fmt::Formatter;
use cosmwasm_schema::serde::de::{SeqAccess, Visitor};
use cosmwasm_schema::serde::ser::SerializeSeq;
use cosmwasm_schema::serde::{Deserialize, Deserializer, Serialize, Serializer};
use schemars::JsonSchema;
use std::collections::BTreeMap;
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq, Default, JsonSchema)]
pub struct SerializableMap<K, V>(BTreeMap<K, V>)
where
    K: Ord + Serialize,
    V: Serialize;

impl<K, V> SerializableMap<K, V>
where
    K: Ord + Serialize,
    V: Serialize,
{
    pub fn new() -> SerializableMap<K, V> {
        Self(BTreeMap::new())
    }

    pub fn from(items: Vec<(K, V)>) -> SerializableMap<K, V> {
        let mut me = Self(BTreeMap::new());
        for item in items {
            me.set(item.0, item.1)
        }
        me
    }

    pub fn set(&mut self, key: K, value: V) {
        self.0.insert(key, value);
    }

    pub fn delete(&mut self, key: &K) -> bool {
        self.0.remove(key).is_some()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<K, V> Serialize for SerializableMap<K, V>
where
    K: Ord + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(self.0.len()))?;
        for (k, v) in self.0.iter() {
            s.serialize_element(&(k, v))?;
        }
        s.end()
    }
}

impl<'d, K, V> Deserialize<'d> for SerializableMap<K, V>
where
    K: Ord + Serialize + Deserialize<'d>,
    V: Serialize + Deserialize<'d>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_seq(SerializableMapVisitor::new())
    }
}

struct SerializableMapVisitor<K, V>
where
    K: Ord + Serialize,
    V: Serialize,
{
    phantom_data: PhantomData<(K, V)>,
}

impl<K, V> SerializableMapVisitor<K, V>
where
    K: Ord + Serialize,
    V: Serialize,
{
    pub fn new() -> SerializableMapVisitor<K, V> {
        SerializableMapVisitor {
            phantom_data: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for SerializableMapVisitor<K, V>
where
    K: Ord + Serialize + Deserialize<'de>,
    V: Serialize + Deserialize<'de>,
{
    type Value = SerializableMap<K, V>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("struct SerializableMap")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut map: SerializableMap<K, V> = SerializableMap::new();
        while let Some(element) = seq.next_element::<(K, V)>()? {
            map.set(element.0, element.1);
        }
        Ok(map)
    }
}
