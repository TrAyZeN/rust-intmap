extern crate rand;

extern crate intmap;

use intmap::{Entry, IntMap};

#[cfg(test)]
mod tests {
    use super::*;

    fn get_random_range(count: usize) -> Vec<u64> {
        use rand::prelude::StdRng;
        use rand::{Rng, SeedableRng};

        let mut vec = Vec::new();
        let mut rng = StdRng::seed_from_u64(4242);

        for _ in 0..count {
            vec.push(rng.gen::<u64>());
        }

        vec.sort();
        vec.dedup();

        vec
    }

    #[test]
    fn intmap_get_insert_impl() {
        let count = 20_000;
        let data = get_random_range(count);
        let mut map: IntMap<u64, u64> = IntMap::new();

        println!();
        println!("Starting test");

        for s in data.iter() {
            assert!(
                map.insert_checked(*s, *s),
                "intmap insert failed! ix: {:?}",
                s
            );
        }

        assert_eq!(map.len(), count);
        assert!(map.assert_count());

        for s in data.iter() {
            assert_eq!(*map.get(*s).unwrap(), *s, "intmap get failed! key: {:?}", s);
        }

        assert_eq!(map.len(), count);

        for s in data.iter() {
            assert!(
                map.contains_key(*s),
                "intmap contains_key failed! key: {:?}",
                s
            );
        }

        assert_eq!(map.len(), count);

        for s in data.iter() {
            let val = map.remove(*s).unwrap();
            assert_eq!(val, *s, "intmap remove failed! key: {:?}", s);
        }

        assert_eq!(map.len(), 0);
        assert!(map.assert_count());
    }

    #[test]
    fn reserve() {
        let mut map: IntMap<u64, bool> = IntMap::new();
        map.reserve(9001);
    }

    #[test]
    fn add_duplicate() {
        let mut map = IntMap::new();

        for i in 0..20_000 {
            assert_eq!(map.insert(i, format!("item: {:?}", i)), None);
            assert_eq!(
                map.insert(i, format!("item: {:?}", i)),
                Some(format!("item: {:?}", i))
            );
        }
    }

    #[test]
    fn add_duplicate_replace() {
        let mut map = IntMap::new();

        for i in 0..20_000 {
            assert!(map.insert_checked(i, format!("item: {:?}", i)));
            assert!(!map.insert_checked(i, format!("item: {:?}", i)));
        }
    }

    #[test]
    fn get_value_map() {
        let mut map = IntMap::new();

        for i in 0..20_000 {
            assert!(map.insert_checked(i, i + 1));
        }

        for i in 0..20_000 {
            assert!(map.contains_key(i));
            assert_eq!(*map.get(i).unwrap(), i + 1);
            assert_eq!(*map.get_mut(i).unwrap(), i + 1);
            assert_eq!(map.remove(i).unwrap(), i + 1);
        }

        for i in 0..20_000 {
            assert!(!map.contains_key(i));
            assert_eq!(map.remove(i), None);
        }

        assert!(map.is_empty());
    }

    #[test]
    fn get_value_not_in_map() {
        let mut map = IntMap::new();

        for i in 0..20_000 {
            assert!(map.insert_checked(i, format!("item: {:?}", i)));
        }

        for i in 20_000..40_000 {
            assert_eq!(map.get(i), None);
            assert_eq!(map.get_mut(i), None);
        }
    }

    #[test]
    fn add_string() {
        let mut map = IntMap::new();

        for i in 0..20_000 {
            map.insert(i, format!("item: {:?}", i));
        }
    }

    #[test]
    fn retain() {
        let mut map = IntMap::new();

        for i in 0..20_000 {
            map.insert(i, format!("item: {:?}", i));
        }

        map.retain(|k, _v| k > 10_000);

        for i in 10_001..20_000 {
            assert!(map.contains_key(i));
        }
    }

    #[test]
    fn single_add_get() {
        let mut map: IntMap<u64, u64> = IntMap::new();
        map.insert(21, 42);
        let val = map.get(21);
        assert!(val.is_some());
        assert_eq!(*val.unwrap(), 42);
    }

    #[test]
    fn map_iter() {
        let count = 20_000;
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for (k, v) in map.iter() {
            assert_eq!(k, *v);
        }
    }

    #[test]
    fn map_iter_keys() {
        let count = 20_000;
        let data: Vec<_> = (0..count).collect();
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for k in map.keys() {
            assert_eq!(k, data[k as usize]);
        }
    }

    #[test]
    fn map_iter_values() {
        let count = 20_000;
        let data: Vec<_> = (0..count).collect();
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for v in map.values() {
            assert_eq!(*v, data[*v as usize]);
        }
    }

    #[test]
    fn map_iter_values_mut() {
        let count = 20_000;
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for value in map.values_mut() {
            *value += 1;
        }

        for n in 0..count {
            assert_eq!(n + 1, *map.get(n).expect("Failed to get number!"));
        }
    }

    #[test]
    fn map_mut_iter() {
        let count = 20_000;
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for kv in map.iter_mut() {
            *kv.1 += 1;
        }

        for n in 0..count {
            assert_eq!(n + 1, *map.get(n).expect("Failed to get number!"));
        }
    }

    #[test]
    fn map_iter_empty() {
        let mut map: IntMap<u64, u64> = IntMap::new();
        map.clear();

        if let Some(kv) = map.iter().next() {
            panic!("Not printing: {:?}", kv);
        }
    }

    #[test]
    fn map_mut_iter_empty() {
        let mut map: IntMap<u64, u64> = IntMap::new();
        map.clear();

        if let Some(kv) = map.iter_mut().next() {
            panic!("Not printing: {:?}", kv);
        }
    }

    #[test]
    fn map_into_iter() {
        let count = 20_000;
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for (k, v) in map.into_iter() {
            assert_eq!(k, v);
        }
    }

    #[test]
    fn map_for() {
        let count = 20_000;
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for (k, v) in &map {
            assert_eq!(k, *v);
        }

        for (k, v) in &mut map {
            assert_eq!(k, *v);
        }

        for (k, v) in map {
            assert_eq!(k, v);
        }
    }

    #[test]
    fn map_drain() {
        let count = 20_000;
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for (k, v) in map.drain() {
            assert_eq!(k, v);
        }
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn map_into_iter_empty() {
        let mut map: IntMap<u64, u64> = IntMap::new();
        map.clear();

        if let Some(kv) = map.into_iter().next() {
            panic!("Not printing: {:?}", kv);
        }
    }

    #[test]
    fn extend_two_maps() {
        let count = 20_000;
        let mut map_1: IntMap<u64, u64> = IntMap::new();
        let mut map_2: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map_1.insert(i, i);
        }

        for i in count..(count * 2) {
            map_2.insert(i, i);
        }

        map_1.extend(map_2);

        assert_eq!(map_1.len(), (count * 2) as usize);

        for (k, &v) in map_1.iter() {
            assert_eq!(k, v);
        }
    }

    #[test]
    fn from_iter_collect() {
        let count = 20_000;

        let map = (0..count).map(|i| (i, i * i)).collect::<IntMap<_, _>>();

        for k in 0..count {
            assert!(map.contains_key(k));
        }

        for (k, &v) in map.iter() {
            assert_eq!(k * k, v);
        }
    }

    #[test]
    fn map_equality() {
        let count = 5_000;

        let map_1 = (0..count).map(|i| (i, i * i)).collect::<IntMap<_, _>>();

        let map_2 = (0..count)
            .rev()
            .map(|i| (i, i * i))
            .collect::<IntMap<_, _>>();

        assert_eq!(map_1, map_2);
    }

    #[test]
    fn map_inequality() {
        let map_1 = (0..10).map(|i| (i, i * i)).collect::<IntMap<_, _>>();
        let map_2 = (0..5).rev().map(|i| (i, i * i)).collect::<IntMap<_, _>>();

        assert_ne!(map_1, map_2);
        assert_ne!(map_2, map_1);
    }

    #[test]
    fn entry_api() {
        let count = 20_000;
        let data: Vec<u64> = (0..count).collect();
        let mut map: IntMap<u64, u64> = IntMap::new();

        // Insert values 0..19999
        for i in 0..count {
            match map.entry(i) {
                Entry::Occupied(_) => panic!("unexpected while insert, i = {}", i),
                Entry::Vacant(entry) => entry.insert(i),
            };
        }

        assert_eq!(map.len(), count as usize);

        for (k, &v) in map.iter() {
            assert_eq!(v, data[k as usize]);
        }

        // Replace values 0..19999 with 20000..39999
        for i in 0..count {
            match map.entry(i) {
                Entry::Occupied(mut entry) => {
                    assert_eq!(*entry.get(), i);
                    assert_eq!(*entry.get_mut(), i);
                    assert_eq!(entry.insert(count + i), i);
                    assert_eq!(*entry.into_mut(), count + i);
                }
                Entry::Vacant(_) => panic!("unexpected while replace, i = {}", i),
            };
        }

        assert_eq!(map.len(), count as usize);

        for (k, &v) in map.iter() {
            assert_eq!(v, count + data[k as usize]);
        }

        // Remove values 20000..39999
        for i in 0..count {
            match map.entry(i) {
                Entry::Occupied(entry) => {
                    assert_eq!(entry.remove(), count + i);
                }
                Entry::Vacant(_) => panic!("unexpected while remove, i = {}", i),
            };
        }

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn entry_or_insert_with() {
        let mut map: IntMap<u64, u64> = IntMap::new();

        map.insert(10, 100);

        let mut triggered = false;
        let v = map.entry(10).or_insert_with(|| {
            triggered = true;
            1337
        });

        assert!(!triggered);
        assert_ne!(*v, 1337);

        let mut triggered = false;
        let v = map.entry(239048).or_insert_with(|| {
            triggered = true;
            42
        });

        assert!(triggered);
        assert_eq!(*v, 42);
    }

    #[test]
    fn entry_or_default() {
        let mut map: IntMap<u64, u64> = IntMap::new();

        let cases = [
            (10, 1337),
            (1028390123, 1337),
            (12098312, 08082934),
            (123981, 1337),
            (23498902348, 1337),
            (2198312093, 12983),
        ];

        let def: u64 = Default::default();

        for (key, value) in cases {
            assert!(!map.contains_key(key));
            let it = map.entry(key).or_default();

            assert_eq!(*it, def);
            assert_eq!(*map.get(key).unwrap(), def);

            let it = map.entry(key).or_insert(value);
            assert_ne!(*it, value);
            assert_eq!(*it, def);

            *it = value;
            let it = map.entry(key).or_default();
            assert_ne!(*it, def);
            assert_eq!(*it, value);
        }
    }

    #[test]
    fn entry_or_insert() {
        let mut map: IntMap<u64, u64> = IntMap::new();

        let cases = [
            (10, 1337, 424242, 69),
            (10280123, 1337, 424242, 19283),
            (12091292, 08082934, 424242, 1029381092),
            (12390331, 1337, 429138, 19283),
            (23492348, 3912, 93925, 12309182),
            (21982093, 12983, 491832, 120398213),
        ];

        for (key, value_1, value_2, value_3) in cases {
            assert!(!map.contains_key(key));
            let it = map.entry(key).or_insert(value_1);

            assert_eq!(*it, value_1);
            assert_eq!(*map.get(key).unwrap(), value_1);

            let it = map.entry(key).or_insert(value_2);
            assert_ne!(*it, value_2);
            assert_eq!(*it, value_1);

            *it = value_3;
            let it = map.entry(key).or_insert(value_2);
            assert_eq!(*it, value_3);
        }
    }

    #[test]
    fn test_debug_features() {
        let count = 20_000;
        let mut map: IntMap<u64, u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        assert_eq!(map.load(), 20_000);
        assert_eq!(map.capacity(), 32_768);
        assert!(map.load_rate() > 0.70);
        assert!(map.collisions().is_empty());

        let mut map: IntMap<u64, u64> = IntMap::new();
        for i in 0..3 {
            map.insert(i, i + 1);
        }

        assert_eq!(format!("{:?}", map), "{0: 1, 1: 2, 2: 3}");
    }

    #[test]
    fn load_factor() {
        let mut map: IntMap<u64, u64> = IntMap::new();

        map.set_load_factor(0.0);
        assert_eq!(map.get_load_factor(), 0.0);

        for i in 0..12 {
            map.insert(i, i);
        }

        assert_eq!(map.capacity(), 16384);
        assert!(map.load_rate() <= 1.);
        assert!(map.collisions().is_empty());

        let mut map: IntMap<u64, u64> = IntMap::new();

        map.set_load_factor(0.1);
        assert_eq!(map.get_load_factor(), 0.1);

        for i in 0..12 {
            map.insert(i, i);
        }

        assert_eq!(map.capacity(), 128);
        assert!(map.load_rate() <= 10.);
        assert!(map.collisions().is_empty());

        let mut map: IntMap<u64, u64> = IntMap::new();

        map.set_load_factor(2.);
        assert_eq!(map.get_load_factor(), 2.);

        for i in 0..16 {
            map.insert(i, i);
        }

        println!("Load {}", map.load_rate());
        println!("factor {:?}", map.collisions());

        assert_eq!(map.capacity(), 8);
        assert!(map.load_rate() <= 200.);
        assert_eq!(format!("{:?}", map.collisions()), "{2: 8}");
    }

    #[test]
    fn insert_after_remove() {
        let mut intmap = IntMap::new();
        let key = 65;
        intmap.insert(key, "foo");
        intmap.remove(key);
        let _ = match intmap.entry(key) {
            intmap::Entry::Occupied(_) => unreachable!(),
            intmap::Entry::Vacant(vacant_entry) => *vacant_entry.insert("bar"),
        };
        assert_eq!(format!("{:?}", intmap), "{65: \"bar\"}");
        assert!(intmap.contains_key(key));
    }

    #[test]
    fn eq() {
        let mut a = IntMap::with_capacity(0);
        let mut b = IntMap::with_capacity(17);
        assert_eq!(a, b);
        a.insert(0, "foo");
        assert_ne!(a, b);
        b.insert(0, "foo");
        assert_eq!(a, b);
        b.insert(17, "bar");
        assert_ne!(a, b);
        a.insert(17, "bar");
        assert_eq!(a, b);
        b.insert(13, "baz");
        assert_ne!(a, b);
        a.insert(13, "bazz");
        assert_ne!(a, b);
        b.insert(13, "bazz");
        assert_eq!(a, b);
        a.remove(17);
        assert_ne!(a, b);
        b.remove(17);
        assert_eq!(a, b);
        a.insert(3, "faz");
        assert_ne!(a, b);
        a.insert(2, "foobar");
        assert_ne!(a, b);
        b.insert(2, "foobar");
        assert_ne!(a, b);
        b.insert(3, "faz");
        assert_eq!(a, b);
        for i in 0..1000 {
            a.insert(i, "foo");
        }
        assert_ne!(a, b);
        for i in 0..1000 {
            b.insert(i, "foo");
        }
        assert_eq!(a, b);
        for i in 0..1000 {
            a.remove(i);
        }
        assert_ne!(a, b);
        for i in 0..1000 {
            b.remove(i);
        }
        assert_eq!(a, b);
    }
}
