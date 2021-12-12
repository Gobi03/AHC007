#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const N: usize = 400;
const M: usize = 1995;

const SIDE: usize = 800;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }
    fn from_usize_pair(p: (usize, usize)) -> Self {
        Coord {
            x: p.0 as isize,
            y: p.1 as isize,
        }
    }

    fn in_field(&self) -> bool {
        (0 <= self.x && self.x <= SIDE as isize) && (0 <= self.y && self.y <= SIDE as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // マンハッタン距離
    // fn specific_distance(&self, that: &Self) -> isize {
    //     (self.x - that.x).abs() + (self.y - that.y).abs()
    // }

    // 問題用のユークリッド距離(di)
    fn specific_distance(&self, that: &Self) -> usize {
        let xx = (self.x as f64 - that.x as f64).powi(2);
        let yy = (self.y as f64 - that.y as f64).powi(2);

        (xx + yy).sqrt().round() as usize
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field())
            .collect()
    }

    fn com_to_delta(com: char) -> Self {
        match com {
            'U' => Coord::new((0, -1)),
            'D' => Coord::new((0, 1)),
            'L' => Coord::new((-1, 0)),
            'R' => Coord::new((1, 0)),
            _ => unreachable!(),
        }
    }

    // 四則演算
    fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }
}

struct Input {
    xy: Vec<Coord>,
    uv: Vec<(usize, usize)>,
    l: Vec<usize>,
}
impl Input {
    fn new(xy: Vec<Coord>, uv: Vec<(usize, usize)>) -> Self {
        Self {
            xy,
            uv,
            l: Vec::with_capacity(M),
        }
    }
}

struct Graph {
    edges: Vec<Vec<usize>>,
    dp: Vec<usize>,
    dp_num: usize, // この値より大きい数はdpのdone値に使われたことがない
}
impl Graph {
    fn new(input: &Input) -> Self {
        let mut edges = vec![vec![]; N];
        for &(u, v) in &input.uv {
            edges[u].push(v);
            edges[v].push(u);
        }

        Self {
            edges,
            dp: vec![0; N],
            dp_num: 0,
        }
    }

    fn is_connected(&mut self) -> bool {
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(0);
        self.dp_num += 1;
        self.dp[0] = self.dp_num;
        let mut cnt = 1;
        while !q.is_empty() {
            let now = q.pop_front().unwrap();
            for &e in &self.edges[now] {
                if self.dp[e] != self.dp_num {
                    cnt += 1;
                    self.dp[e] = self.dp_num;
                    q.push_back(e);
                }
            }
        }

        cnt == N
    }

    fn del_edge(&mut self, u: usize, v: usize) {
        self.edges[u].retain(|e| *e != v);
        self.edges[v].retain(|e| *e != u);
    }
    fn add_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v);
        self.edges[v].push(u);
    }
}

fn connect(u: usize, v: usize, graph: &mut Graph, uf: &mut UnionFind) {
    println!("{}", 1);
    graph.add_edge(u, v);
    uf.connect(u, v);
}

fn main() {
    let system_time = SystemTime::now();
    let mut _rng = thread_rng();

    // input
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let mut xy = Vec::with_capacity(N);
    let mut uv = Vec::with_capacity(M);
    for _ in 0..N {
        let x: usize = sc.read();
        let y: usize = sc.read();
        xy.push(Coord::from_usize_pair((x, y)));
    }
    for _ in 0..M {
        let u: usize = sc.read();
        let v: usize = sc.read();
        uv.push((u, v));
    }

    let mut input = Input::new(xy, uv);

    // main
    let mut uf = UnionFind::new();
    let mut graph = Graph::new(&input);
    let mut edge_num = 0;
    let mut kruskal = Kruskal::new(&input, UnionFind::new(), 0);

    // main loop
    for mi in 0..M {
        let l: usize = sc.read();
        input.l.push(l);

        let (u, v) = input.uv[mi];

        // // 2*di 以下が対象
        let di = input.xy[u].specific_distance(&input.xy[v]);

        graph.del_edge(u, v);
        // ここを外すと確定で非連結になってしまう場合
        if !graph.is_connected() {
            connect(u, v, &mut graph, &mut uf);
            edge_num += 1;
        } else {
            if kruskal.d[mi] {
                let degree_min = graph.edges[u].len().min(graph.edges[v].len());
                // let vol = 1.0 + 2.0 * (1.0 / degree_min as f64);
                let vol = 2.5; // TODO: 調整
                if l <= (di as f64 * vol) as usize {
                    connect(u, v, &mut graph, &mut uf);
                    edge_num += 1;
                } else {
                    let new_kruskal = Kruskal::new(&input, uf.clone(), mi);

                    // TODO: 良くなりそうな場合だけ採択
                    kruskal = new_kruskal;
                    println!("{}", 0);
                }
            } else {
                println!("{}", 0);
            }
        }
    }

    eprintln!("{}", edge_num);
    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}

struct Kruskal {
    d: Vec<bool>, // trueが採択
}
impl Kruskal {
    fn new(
        input: &Input,
        mut uf: UnionFind, // 暫定uf
        mi: usize,         // このmi以降で考える
    ) -> Self {
        let mut vs = Vec::with_capacity(M);
        for i in mi..M {
            let (u, v) = input.uv[i];
            if !uf.is_connect(u, v) {
                let di = input.xy[u].specific_distance(&input.xy[v]);
                vs.push((di, i, u, v))
            }
        }

        let mut d = vec![false; M];
        vs.sort();

        for (dist, i, u, v) in vs {
            if !uf.is_connect(u, v) {
                uf.connect(u, v);
                d[i] = true;
            }
        }

        Self { d }
    }
}

#[derive(Clone)]
struct UnionFind {
    uni: Vec<isize>, // 根であれば *そのグループの要素数(負)* が、子であれば親の番号が入る。
}

#[allow(dead_code)]
impl UnionFind {
    fn new() -> Self {
        UnionFind { uni: vec![-1; N] }
    }

    // 頂点 v の所属するグループを調べる
    fn root(&mut self, v: usize) -> usize {
        if self.uni[v] < 0 {
            v
        } else {
            self.uni[v] = self.root(self.uni[v] as usize) as isize;
            self.uni[v] as usize
        }
    }

    // 頂点 a と頂点 b を繋ぐ。元々同じグループのとき　false を返す
    fn connect(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.root(a) as usize;
        let mut root_b = self.root(b) as usize;

        if root_a == root_b {
            return false;
        }

        // a 側が大きいグループになるようにスワップ
        if self.uni[root_a] > self.uni[root_b] {
            root_a ^= root_b;
            root_b ^= root_a;
            root_a ^= root_b;
        }

        // root_a と root_b を結合し、root_b の親を root_a とする
        self.uni[root_a] += self.uni[root_b];
        self.uni[root_b] = root_a as isize;

        return true;
    }

    // 頂点 a, b が同じグループであるかを調べる
    fn is_connect(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    // 頂点 v を含むグループの頂点数を調べる
    fn size(&mut self, v: usize) -> usize {
        let root = self.root(v);
        self.uni[root].abs() as usize
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        self.1.write(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
