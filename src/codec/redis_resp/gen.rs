//  rpc-perf - RPC Performance Testing
//  Copyright 2015 Twitter, Inc
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[cfg(feature = "unstable")]
    #[allow(unused_imports)]
    use test;

    #[test]
    fn test_flushall() {
        assert_eq!(flushall(), "*1\r\n$8\r\nflushall\r\n");
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn flushall_benchmark(b: &mut test::Bencher) {
        b.iter(|| flushall());
    }

    #[test]
    fn test_select() {
        assert_eq!(select(&1), "*2\r\n$6\r\nselect\r\n$1\r\n1\r\n");
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn select_benchmark(b: &mut test::Bencher) {
        b.iter(|| select(&1));
    }

    #[test]
    fn test_set() {
        assert_eq!(
            set("key", "value"),
            "*3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn set_benchmark(b: &mut test::Bencher) {
        b.iter(|| set("key", "value"));
    }

    #[test]
    fn test_hset() {
        assert_eq!(
            hset("hash", "key", "value"),
            "*4\r\n$4\r\nhset\r\n$4\r\nhash\r\n$3\r\nkey\r\n$5\r\nvalue\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn hset_benchmark(b: &mut test::Bencher) {
        b.iter(|| hset("hash", "key", "value"));
    }

    #[test]
    fn test_get() {
        assert_eq!(get("key"), "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n");
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn get_benchmark(b: &mut test::Bencher) {
        b.iter(|| get("key"));
    }

    #[test]
    fn test_hget() {
        assert_eq!(
            hget("hash", "key"),
            "*3\r\n$4\r\nhget\r\n$4\r\nhash\r\n$3\r\nkey\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn hget_benchmark(b: &mut test::Bencher) {
        b.iter(|| hget("hash", "key"));
    }

    #[test]
    fn test_del() {
        assert_eq!(del("key"), "*2\r\n$3\r\ndel\r\n$3\r\nkey\r\n");
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn del_benchmark(b: &mut test::Bencher) {
        b.iter(|| del("key"));
    }

    #[test]
    fn test_incr() {
        assert_eq!(incr("key"), "*2\r\n$4\r\nincr\r\n$3\r\nkey\r\n");
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn incr_benchmark(b: &mut test::Bencher) {
        b.iter(|| incr("key"));
    }

    #[test]
    fn test_decr() {
        assert_eq!(decr("key"), "*2\r\n$4\r\ndecr\r\n$3\r\nkey\r\n");
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn decr_benchmark(b: &mut test::Bencher) {
        b.iter(|| decr("key"));
    }

    #[test]
    fn test_expire() {
        assert_eq!(
            expire("key", 1000),
            "*3\r\n$6\r\nexpire\r\n$3\r\nkey\r\n$4\r\n1000\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn expire_benchmark(b: &mut test::Bencher) {
        b.iter(|| expire("key", 1000));
    }

    #[test]
    fn test_append() {
        assert_eq!(
            append("key", "value"),
            "*3\r\n$6\r\nappend\r\n$3\r\nkey\r\n$5\r\nvalue\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn append_benchmark(b: &mut test::Bencher) {
        b.iter(|| append("key", "value"));
    }

    #[test]
    fn test_prepend() {
        assert_eq!(
            prepend("key", "value"),
            "*3\r\n$7\r\nprepend\r\n$3\r\nkey\r\n$5\r\nvalue\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn prepend_benchmark(b: &mut test::Bencher) {
        b.iter(|| prepend("key", "value"));
    }

    #[test]
    fn test_eval() {
        assert_eq!(
            eval("redis.call(\"set\", KEYS[1], ARGV[1])", vec!["foo", "bar"]),
            "*5\r\n$4\r\neval\r\n$39\r\n\"redis.call(\\\"set\\\", KEYS[1], ARGV[1])\"\r\n$1\r\n1\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"
        );
        assert_eq!(
            eval("redis.call(\"set\", KEYS[1], ARGV[1])\r\nredis.call(\"set\", KEYS[2], ARGV[2])", vec!["foo", "bar", "baz", "toto"]),
            "*7\r\n$4\r\neval\r\n$78\r\n\"redis.call(\\\"set\\\", KEYS[1], ARGV[1])\r\nredis.call(\\\"set\\\", KEYS[2], ARGV[2])\"\r\n$2\r\n2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$3\r\nbaz\r\n$4\r\ntoto\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn eval_benchmark(b: &mut test::Bencher) {
        b.iter(|| eval("redis.call(\"set\", KEYS[1], ARGV[1])", vec!["foo", "bar"]))
    }

    #[test]
    fn test_script_load() {
        assert_eq!(
            script_load("redis.call(\"set\", KEYS[1], ARGV[1])"),
            "*3\r\n$6\r\nscript\r\n$4\r\nload\r\n$39\r\n\"redis.call(\\\"set\\\", KEYS[1], ARGV[1])\"\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn script_load_benchmark(b: &mut test::Bencher) {
        b.iter(|| script_load("redis.call(\"set\", KEYS[1], ARGV[1])"));
    }

    #[test]
    fn test_script_flush() {
        assert_eq!(
            script_flush(),
            "*2\r\n$6\r\nscript\r\n$5\r\nflush\r\n",
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn script_flush_benchmark(b: &mut test::Bencher) {
        b.iter(|| script_flush());
    }

    #[test]
    fn test_evalsha() {
        assert_eq!(
            evalsha("MYSTERIOUS_SHA", vec!["foo", "bar"]),
            "*5\r\n$7\r\nevalsha\r\n$14\r\nMYSTERIOUS_SHA\r\n$1\r\n1\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"
        );
        assert_eq!(
            evalsha("MYSTERIOUS_SHA", vec!["foo", "bar", "baz", "toto"]),
            "*7\r\n$7\r\nevalsha\r\n$14\r\nMYSTERIOUS_SHA\r\n$2\r\n2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$3\r\nbaz\r\n$4\r\ntoto\r\n"
        );
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn evalsha_benchmark(b: &mut test::Bencher) {
        b.iter(|| evalsha("MYSTERIOUS_SHA", vec!["foo", "bar"]));
    }
}

/// FLUSHALL request
pub fn flushall() -> String {
    "*1\r\n$8\r\nflushall\r\n".to_owned()
}

/// SELECT request
pub fn select(database: &u32) -> String {
    let database = format!("{}", database);
    format!(
        "*2\r\n$6\r\nselect\r\n${}\r\n{}\r\n",
        database.len(),
        database
    )
}

/// SET request
pub fn set(key: &str, value: &str) -> String {
    format!(
        "*3\r\n$3\r\nset\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
        key.len(),
        key,
        value.len(),
        value
    )
}

/// HSET request
pub fn hset(hash: &str, key: &str, value: &str) -> String {
    format!(
        "*4\r\n$4\r\nhset\r\n${}\r\n{}\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
        hash.len(),
        hash,
        key.len(),
        key,
        value.len(),
        value
    )
}

/// GET request
pub fn get(key: &str) -> String {
    format!("*2\r\n$3\r\nget\r\n${}\r\n{}\r\n", key.len(), key)
}

/// HGET request
pub fn hget(hash: &str, key: &str) -> String {
    format!(
        "*3\r\n$4\r\nhget\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
        hash.len(),
        hash,
        key.len(),
        key
    )
}

/// DEL request
pub fn del(key: &str) -> String {
    format!("*2\r\n$3\r\ndel\r\n${}\r\n{}\r\n", key.len(), key)
}

/// INCR request
pub fn incr(key: &str) -> String {
    format!("*2\r\n$4\r\nincr\r\n${}\r\n{}\r\n", key.len(), key)
}

/// DECR request
pub fn decr(key: &str) -> String {
    format!("*2\r\n$4\r\ndecr\r\n${}\r\n{}\r\n", key.len(), key)
}

/// EXPIRE request
pub fn expire(key: &str, ttl: u32) -> String {
    let ttl = format!("{}", ttl);
    format!(
        "*3\r\n$6\r\nexpire\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
        key.len(),
        key,
        ttl.len(),
        ttl
    )
}

/// APPEND request
pub fn append(key: &str, value: &str) -> String {
    format!(
        "*3\r\n$6\r\nappend\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
        key.len(),
        key,
        value.len(),
        value
    )
}

/// PREPEND request
pub fn prepend(key: &str, value: &str) -> String {
    format!(
        "*3\r\n$7\r\nprepend\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
        key.len(),
        key,
        value.len(),
        value
    )
}

/// EVAL request
pub fn eval(script: &str, keys: Vec<&str>) -> String {
    let escaped_script = format!("\"{}\"", script.replace("\"", "\\\""));

    format!(
        "*{}\r\n$4\r\neval\r\n${}\r\n{}\r\n${}\r\n{}\r\n{}",
        3 + keys.len(),
        escaped_script.len(),
        escaped_script,
        keys.len() / 2 % 10,
        keys.len() / 2,
        keys.iter()
            .map(|k| format!("${}\r\n{}\r\n", k.len(), k))
            .collect::<Vec<String>>()
            .join(""),
    )
}

/// EVALSHA request
pub fn evalsha(sha: &str, keys: Vec<&str>) -> String {
    format!(
        "*{}\r\n$7\r\nevalsha\r\n${}\r\n{}\r\n${}\r\n{}\r\n{}",
        3 + keys.len(),
        sha.len(),
        sha,
        keys.len() / 2 % 10,
        keys.len() / 2,
        keys.iter()
            .map(|k| format!("${}\r\n{}\r\n", k.len(), k))
            .collect::<Vec<String>>()
            .join(""),
    )
}

/// SCRIPT LOAD request
pub fn script_load(script: &str) -> String {
    let escaped_script = format!("\"{}\"", script.replace("\"", "\\\""));
    format!(
        "*3\r\n$6\r\nscript\r\n$4\r\nload\r\n${}\r\n{}\r\n",
        escaped_script.len(),
        escaped_script
    )
}

/// SCRIPT FLUSH request
pub fn script_flush() -> String {
    format!("*2\r\n$6\r\nscript\r\n$5\r\nflush\r\n")
}
