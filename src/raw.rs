use crate::{Config, Error, Key, KeyRef, KvPair, Value};
use futures::{Future, Poll};
use std::{
    ops::{RangeBounds, Bound::{self, *}},
    marker::PhantomData,
};

/// A [`ColumnFamily`](struct.ColumnFamily.html) is an optional parameter for [`raw::Client`](struct.Client.html) requests.
/// 
/// TiKV uses RocksDB's `ColumnFamily` support. You can learn more about RocksDB's `ColumnFamily`s [on their wiki](https://github.com/facebook/rocksdb/wiki/Column-Families).
/// 
/// By default in TiKV data is stored in three different `ColumnFamily` values, configurable in the TiKV server's configuration:
/// 
/// * Default: Where real user data is stored. Set by `[rocksdb.defaultcf]`.
/// * Write: Where MVCC and index related data are stored. Set by `[rocksdb.writecf]`.
/// * Lock: Where lock information is stored. Set by `[rocksdb.lockcf]`.
/// 
/// Not providing a call a `ColumnFamily` means it will use the default value of `default`.
/// 
/// The best (and only) way to create a [`ColumnFamily`](struct.ColumnFamily.html) is via the `From` implementation:
/// 
/// ```rust
/// # use tikv_client::raw::ColumnFamily;
/// let cf = ColumnFamily::from("write");
/// let cf = ColumnFamily::from(String::from("write"));
/// let cf = ColumnFamily::from(&String::from("write"));
/// ```
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ColumnFamily(String);

impl<T> From<T> for ColumnFamily
where
    T: ToString,
{
    fn from(i: T) -> ColumnFamily {
        ColumnFamily(i.to_string())
    }
}

/// A raw request to get the [`Value`](struct.Value.html) of a given [`Key`](struct.key.html).
pub struct Get<'client, 'key: 'client> {
    client: &'client Client,
    key: KeyRef<'key>,
    cf: Option<ColumnFamily>,
}

impl<'client, 'key: 'client> Get<'client, 'key> {
    fn new(client: &'client Client, key: KeyRef<'key>) -> Self {
        Get {
            client,
            key,
            cf: None,
        }
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }
}

impl<'client, 'key: 'client> Future for Get<'client, 'key> {
    type Item = Value;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.key;
        let _ = &self.cf;
        unimplemented!()
    }
}

pub struct BatchGet<'client, 'keys: 'client, Iter>
where Iter: Iterator<Item=KeyRef<'keys>> {
    client: &'client Client,
    keys: Iter,
    cf: Option<ColumnFamily>,
}

impl<'client, 'keys: 'client, Iter> BatchGet<'client, 'keys, Iter> 
where Iter: Iterator<Item=KeyRef<'keys>> {
    fn new(client: &'client Client, keys: Iter) -> Self {
        BatchGet {
            client,
            keys,
            cf: None,
        }
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }
}

impl<'client, 'keys: 'client, Iter> Future for BatchGet<'client, 'keys, Iter> 
where Iter: Iterator<Item=KeyRef<'keys>> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.keys;
        let _ = &self.cf;
        unimplemented!()
    }
}

pub struct Put<'a> {
    client: &'a Client,
    key: Key,
    value: Value,
    cf: Option<ColumnFamily>,
}

impl<'a> Put<'a> {
    fn new(client: &'a Client, key: Key, value: Value) -> Self {
        Put {
            client,
            key,
            value,
            cf: None,
        }
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }
}

impl<'a> Future for Put<'a> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.key;
        let _ = &self.value;
        let _ = &self.cf;
        unimplemented!()
    }
}

pub struct BatchPut<'a> {
    client: &'a Client,
    pairs: Vec<KvPair>,
    cf: Option<ColumnFamily>,
}

impl<'a> BatchPut<'a> {
    fn new(client: &'a Client, pairs: Vec<KvPair>) -> Self {
        BatchPut {
            client,
            pairs,
            cf: None,
        }
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }
}

impl<'a> Future for BatchPut<'a> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.pairs;
        let _ = &self.cf;
        unimplemented!()
    }
}

/// An unresolved delete request.
/// 
/// Once resolved this request will result in the deletion of the given key.
/// 
/// ```rust,no_run
/// use tikv_client::{Config, raw::Client};
/// use futures::Future;
/// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
/// let connected_client = connecting_client.wait().unwrap();
/// let key = b"TiKV";
/// let delete_req = connected_client.delete(key.as_ref());
/// delete_req.wait();
/// ```
pub struct Delete<'client, 'key: 'client> {
    client: &'client Client,
    key: KeyRef<'key>,
    cf: Option<ColumnFamily>,
}

impl<'client, 'key: 'client> Delete<'client, 'key> {
    fn new(client: &'client Client, key: KeyRef<'key>) -> Self {
        Delete {
            client,
            key,
            cf: None,
        }
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }
}

impl<'client, 'key: 'client> Future for Delete<'client, 'key> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.key;
        let _ = &self.cf;
        unimplemented!()
    }
}

pub struct BatchDelete<'client, 'keys: 'client, Iter>
where Iter: Iterator<Item=KeyRef<'keys>> {
    client: &'client Client,
    keys: Iter,
    cf: Option<ColumnFamily>,
}

impl<'client, 'keys: 'client, Iter> BatchDelete<'client, 'keys, Iter> 
where Iter: Iterator<Item=KeyRef<'keys>> {
    fn new(client: &'client Client, keys: Iter) -> Self {
        BatchDelete {
            client,
            keys,
            cf: None,
        }
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }
}

impl<'client, 'keys: 'client, Iter> Future for BatchDelete<'client, 'keys, Iter> 
where Iter: Iterator<Item=KeyRef<'keys>> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.keys;
        let _ = &self.cf;
        unimplemented!()
    }
}

pub struct Scan<'client, 'keys: 'client, Bounds> where Bounds: RangeBounds<KeyRef<'keys>> {
    client: &'client Client,
    range: Bounds,
    range_marker: &'keys PhantomData<Bounds>,
    limit: u32,
    key_only: bool,
    cf: Option<ColumnFamily>,
    reverse: bool,
}

impl<'client, 'keys: 'client, Bounds> Scan<'client, 'keys, Bounds>
where Bounds: RangeBounds<KeyRef<'keys>>{
    fn new(client: &'client Client, range: Bounds, limit: u32) -> Self {
        Scan {
            client,
            range,
            range_marker: &PhantomData,
            limit,
            key_only: false,
            cf: None,
            reverse: false,
        }
    }

    pub fn key_only(mut self) -> Self {
        self.key_only = true;
        self
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
}

impl<'client, 'keys: 'client, Bounds> Future for Scan<'client, 'keys, Bounds>
where Bounds: RangeBounds<KeyRef<'keys>> {
    type Item = Vec<KvPair>;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.range;
        let _ = &self.limit;
        let _ = &self.key_only;
        let _ = &self.cf;
        unimplemented!()
    }
}

pub struct BatchScan<'client, 'keys: 'client, Bounds, Iter>
where Bounds: RangeBounds<KeyRef<'keys>>, Iter: Iterator<Item=Bounds> {
    client: &'client Client,
    ranges: Iter,
    ranges_marker: &'keys PhantomData<Bounds>,
    each_limit: u32,
    key_only: bool,
    cf: Option<ColumnFamily>,
    reverse: bool,
}

impl<'client, 'keys: 'client, Bounds, Iter> BatchScan<'client, 'keys, Bounds, Iter>
where Bounds: RangeBounds<KeyRef<'keys>>, Iter: Iterator<Item=Bounds> {
    fn new(client: &'client Client, ranges: Iter, each_limit: u32) -> Self {
        BatchScan {
            client,
            ranges,
            ranges_marker: &PhantomData,
            each_limit,
            key_only: false,
            cf: None,
            reverse: false,
        }
    }

    pub fn key_only(mut self) -> Self {
        self.key_only = true;
        self
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
}

impl<'client, 'keys: 'client, Bounds, Iter> Future for BatchScan<'client, 'keys, Bounds, Iter> 
where Bounds: RangeBounds<KeyRef<'keys>>, Iter: Iterator<Item=Bounds> {
    type Item = Vec<KvPair>;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.ranges;
        let _ = &self.each_limit;
        let _ = &self.key_only;
        let _ = &self.cf;
        unimplemented!()
    }
}

pub struct DeleteRange<'client, 'keys: 'client, Bounds> where Bounds: RangeBounds<KeyRef<'keys>> {
    client: &'client Client,
    range: Bounds,
    range_marker: &'keys PhantomData<Bounds>,
    cf: Option<ColumnFamily>,
}

impl<'client, 'keys, Bounds> DeleteRange<'client, 'keys, Bounds>
where Bounds: RangeBounds<KeyRef<'keys>> {
    fn new(client: &'client Client, range: Bounds) -> Self {
        DeleteRange {
            client,
            range,
            range_marker: &PhantomData,
            cf: None,
        }
    }

    /// Set the (optional) [`ColumnFamily`](struct.ColumnFamily.html).
    pub fn cf(mut self, cf: impl Into<ColumnFamily>) -> Self {
        self.cf = Some(cf.into());
        self
    }
}

impl<'client, 'keys, Bounds> Future for DeleteRange<'client, 'keys, Bounds>
where Bounds: RangeBounds<KeyRef<'keys>> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _ = &self.client;
        let _ = &self.range;
        let _ = &self.cf;
        unimplemented!()
    }
}

/// A future which resolves the initial connection between the [`Client`](struct.Client.html) and the TiKV cluster.
/// 
/// ```rust,no_run
/// # use tikv_client::{Config, raw::{Client, Connect}};
/// # use futures::Future;
/// let connect = Client::new(&Config::default());
/// let client = connect.wait();
/// ```
pub struct Connect {
    config: Config,
}

impl Connect {
    fn new(config: Config) -> Self {
        Connect { config }
    }
}

impl Future for Connect {
    type Item = Client;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let _config = &self.config;
        unimplemented!()
    }
}

/// The TiKV raw [`Client`](struct.Client.html) is used to issue requests to the TiKV server and PD cluster.
pub struct Client;

impl Client {
    #![cfg_attr(feature = "cargo-clippy", allow(clippy::new_ret_no_self))]
    /// Create a new [`Client`](struct.Client.html) once the [`Connect`](struct.Connect.html) resolves.
    /// 
    /// ```rust,no_run
    /// # use tikv_client::{Config, raw::{Client, Connect}};
    /// # use futures::Future;
    /// let connect = Client::new(&Config::default());
    /// let client = connect.wait();
    /// ```
    pub fn new(config: &Config) -> Connect {
        Connect::new(config.clone())
    }

    /// Create a new [`Get`](struct.Get.html) request.
    ///
    /// Once resolved this request will result in the fetching of the value associated with the given key.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let key = &b"TiKV"[..];
    /// let req = connected_client.get(key);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let key = String::from("TiKV");
    /// let req = connected_client.get(&key);
    /// 
    /// let key = "TiKV";
    /// let req = connected_client.get(key);
    /// ```
    pub fn get<'client, 'key: 'client>(&'client self, key: impl Into<KeyRef<'key>>) -> Get<'client, 'key> {
        Get::new(self, key.into())
    }

    /// Create a new [`BatchGet`](struct.BatchGet.html) request.
    /// 
    /// Once resolved this request will result in the fetching of the values associated with the given keys.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let keys = vec![&b"TiKV"[..], &b"TiDB"[..]];
    /// let req = connected_client.batch_get(keys);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let keys = vec!["TiKV", "TiDB"];
    /// let req = connected_client.batch_get(keys);
    /// 
    /// let (string1, string2) = (String::from("TiKV"), String::from("TiDB"));
    /// let keys = vec![&string1, &string2];
    /// let req = connected_client.batch_get(keys);
    /// ```
    pub fn batch_get<'client, 'keys: 'client>(&'client self, keys: impl IntoIterator<Item=impl Into<KeyRef<'keys>>>) 
    -> BatchGet<'client, 'keys, impl Iterator<Item=KeyRef<'keys>>> {
        BatchGet::new(self, keys.into_iter().map(Into::into))
    }

    /// Create a new [`Put`](struct.Put.html) request.
    ///
    /// Once resolved this request will result in the setting of the value associated with the given key.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{Key, Value, Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let key = Key::from(b"TiKV".to_vec());
    /// let val = Value::from(b"TiKV".to_vec());
    /// let req = connected_client.put(key, val);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let key = String::from("TiKV");
    /// let val = String::from("Client");
    /// let req = connected_client.put(key, val);
    /// 
    /// let key = b"TiKV".to_vec();
    /// let val = b"Client".to_vec();
    /// let req = connected_client.put(key, val);
    /// ```
    pub fn put(&self, key: impl Into<Key>, value: impl Into<Value>) -> Put {
        Put::new(self, key.into(), value.into())
    }
    
    /// Create a new [`BatchPut`](struct.BatchPut.html) request.
    ///
    /// Once resolved this request will result in the setting of the value associated with the given key.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{KvPair, Key, Value, Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:

    /// let kvpair1 = KvPair::from((Key::from(b"TiDB".to_vec()), Value::from(b"Go".to_vec())));
    /// let kvpair2 = KvPair::from((Key::from(b"TiDB".to_vec()), Value::from(b"Go".to_vec())));
    /// let req = connected_client.batch_put(vec![kvpair1, kvpair2]);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let kvpair1 = KvPair::from((b"TiKV".to_vec(), b"Rust".to_vec()));
    /// let kvpair2 = KvPair::from((b"TiKV".to_vec(), b"Rust".to_vec()));
    /// let req = connected_client.batch_put(vec![kvpair1, kvpair2]);
    /// 
    /// let kvpairs = vec![
    ///     (String::from("TiKV"), String::from("Client")),
    ///     (String::from("TiKV"), String::from("Client")),
    /// ];
    /// let req = connected_client.batch_put(kvpairs);
    /// 
    /// let req = connected_client.batch_put(vec![
    ///     (b"TiKV".to_vec(), b"Rust".to_vec()),
    ///     (b"TiDB".to_vec(), b"Go".to_vec()),
    /// ]);
    /// ```
    pub fn batch_put(&self, pairs: impl IntoIterator<Item = impl Into<KvPair>>) -> BatchPut {
        BatchPut::new(self, pairs.into_iter().map(Into::into).collect())
    }

    /// Create a new [`Delete`](struct.Delete.html) request.
    /// 
    /// Once resolved this request will result in the deletion of the given key.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{KeyRef, Key, Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let key = KeyRef::from(&b"TiKV"[..]);
    /// let req = connected_client.delete(key);
    /// req.wait();
    /// 
    /// // Other possibilities:
    // let key = Key::from(&b"TiKV[..]);
    // let req = connected_client.delete(&key);
    /// 
    /// let key = &b"TiKV"[..];
    /// let req = connected_client.delete(key);
    /// 
    /// let key = String::from("TiKV");
    /// let req = connected_client.delete(&key);
    /// 
    /// let key = "TiKV";
    /// let req = connected_client.delete(key);
    /// ```
    pub fn delete<'client, 'key: 'client>(&'client self, key: impl Into<KeyRef<'key>>) -> Delete<'client, 'key> {
        Delete::new(self, key.into())
    }

    /// Create a new [`BatchDelete`](struct.BatchDelete.html) request.
    /// 
    /// Once resolved this request will result in the deletion of the given keys.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let keys = vec![&b"TiKV"[..], &b"TiDB"[..]];
    /// let req = connected_client.batch_delete(keys);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let keys = vec!["TiKV", "TiDB"];
    /// let req = connected_client.batch_delete(keys);
    /// 
    /// let (string1, string2) = (String::from("TiKV"), String::from("TiDB"));
    /// let keys = vec![&string1, &string2];
    /// let req = connected_client.batch_delete(keys);
    /// 
    /// let (key1, string2) = (String::from("TiKV"), String::from("TiDB"));
    /// let keys = vec![&string1, &string2];
    /// let req = connected_client.batch_delete(keys);
    /// ```
    pub fn batch_delete<'client, 'keys: 'client>(&'client self, keys: impl IntoIterator<Item=impl Into<KeyRef<'keys>>>) 
    -> BatchDelete<'client, 'keys, impl Iterator<Item=KeyRef<'keys>>> {
        BatchDelete::new(self, keys.into_iter().map(Into::into))
    }

    /// Create a new [`Scan`](struct.Scan.html) request.
    /// 
    /// Once resolved this request will result in a scanner over the given keys.
    /// 
    /// If not passed a `limit` parameter, it will default to `u32::MAX`.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{KeyRef, Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let inclusive_range = KeyRef::from("TiKV")..=KeyRef::from("TiDB");
    /// let req = connected_client.scan(inclusive_range, 2);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let exclusive_range = KeyRef::from("TiKV")..KeyRef::from("TiDB");
    /// let req = connected_client.scan(exclusive_range, None);
    /// ```
    pub fn scan<'client, 'keys, Bounds>(&'client self, range: Bounds, limit: impl Into<Option<u32>>) -> Scan<'client, 'keys, Bounds> 
    where Bounds: RangeBounds<KeyRef<'keys>> {
        use std::u32::MAX;
        Scan::new(self, range, limit.into().unwrap_or(MAX))
    }

    /// Create a new [`BatchScan`](struct.BatchScan.html) request.
    /// 
    /// Once resolved this request will result in a set of scanners over the given keys.
    /// 
    /// If not passed a `limit` parameter, it will default to `u32::MAX`.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{KeyRef, Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let inclusive_range1 = KeyRef::from("TiDB")..=KeyRef::from("TiKV");
    /// let inclusive_range2 = KeyRef::from("TiKV")..=KeyRef::from("TiSpark");
    /// let req = connected_client.batch_scan(vec![inclusive_range1, inclusive_range2], 2);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let exclusive_range1 = KeyRef::from("TiDB")..KeyRef::from("TiKV");
    /// let exclusive_range2 = KeyRef::from("TiKV")..KeyRef::from("TiSpark");
    /// let req = connected_client.batch_scan(vec![exclusive_range1, exclusive_range2], None);
    /// ```
    pub fn batch_scan<'client, 'keys, Bounds>(&'client self, ranges: impl IntoIterator<Item=Bounds>, each_limit: impl Into<Option<u32>>) -> BatchScan<'client, 'keys, Bounds, impl Iterator<Item=Bounds>>
    where Bounds: RangeBounds<KeyRef<'keys>> {
        use std::u32::MAX;
        BatchScan::new(
            self,
            ranges.into_iter(),
            each_limit.into().unwrap_or(MAX),
        )
    }

    /// Create a new [`DeleteRange`](struct.DeleteRange.html) request.
    /// 
    /// Once resolved this request will result in the deletion of all keys over the given range.
    /// 
    /// If not passed a `limit` parameter, it will default to `u32::MAX`.
    /// 
    /// ```rust,no_run
    /// use tikv_client::{KeyRef, Config, raw::Client};
    /// use futures::Future;
    /// let connecting_client = Client::new(&Config::new(vec!["192.168.0.100", "192.168.0.101"]));
    /// let connected_client = connecting_client.wait().unwrap();
    /// // This is the most explicit form:
    /// let inclusive_range = KeyRef::from("TiKV")..=KeyRef::from("TiDB");
    /// let req = connected_client.delete_range(inclusive_range);
    /// req.wait();
    /// 
    /// // Other possibilities:
    /// let exclusive_range = KeyRef::from("TiKV")..KeyRef::from("TiDB");
    /// let req = connected_client.delete_range(exclusive_range);
    /// ```
    pub fn delete_range<'client, 'keys, Bounds>(&'client self, range: Bounds) -> DeleteRange<'client, 'keys, Bounds>
    where Bounds: RangeBounds<KeyRef<'keys>> {
        DeleteRange::new(self, range)
    }
}
