# advent-of-code-2021

## 需要重新考虑的

部分题目我采用了非常规的做法（day 18），或者用了题目输入输出的偶然性（day 19）而完成。这些题目我需要进一步的重新编写。记录中的题目虽然大部分都是有所不足的，但是这些不足更多的是在实现上有效率的问题，大体上是不存在严重问题的，而列表内的题目则是方法有误，所以最好能进行重写。

- Day 18
- Day 19

## 记录

### Day 14

最初的想法是将规则存入一个HashMap，而将聚合物存入列表。对于每一次聚合，遍历聚合物的两个元素，根据元素从规则中找到新的三个聚合元素，并存入新的聚合物列表，遍历结束后再将新的聚合物列表替换原有的聚合物列表。

**复杂度分析：** 这个实现方式的效率极低，因为每次遍历的次数都是前一次的两倍，实际上是一个指数级的增长。那么对于N个元素，M次聚合，第N次需要至少遍历2^n个元素，总共的复杂度应该是 O(M*2^N)。

在无法快速取得结果后，我尝试观察每一次的结果，而花了大量的时间尝试寻找其中的规律。最后我只好求助于社区的答案，于是在twitter上找到了 [dcreemer/adventofcode](https://github.com/dcreemer/adventofcode) 的答案。我根据我看到的解决办法，利用 Rust 写了新的答案。

同样是将规则存入HashMap，但是这次以键是聚合物对，值是聚合物对出现的次数，将聚合物存入HashMap。对于每一次聚合，新建一个与聚合物相同的HashMap，遍历聚合物的两个元素，也就是一个聚合对，根据规则实际上一个聚合对在一次聚合之后，会生成两个新的聚合，那么新生成的两个聚合对的次数也就是旧友聚合对的次数，将新的聚合对和次数存入新的HashMap。每次聚合结束之后，再替换旧的聚合物HashMap为新的HashMap。

**复杂度分析：** 对于输入，总有10个元素，那么总共有100个元素对，对于每次聚合，元素对的总数不变，也就是只需要进行200次的HashMap插入操作。那么对于N个元素，M次聚合，总共的复杂度应该是 O(MN^2) 。这个方法的复杂度较我自己的方法是一个指数时间和二次时间的差别。

**对于计数的复杂度分析：** 利用一个键为元素，值为个数的HashMap来对元素对出现次数进行统计，对于每一次聚合中的每一个聚合对，实际上只是增加了一个元素，那么在这时对HashMap的元素次数加一即可。总共进行 O(2^N) 次HashMap次数更新。

### Day 15:

使用 Dijkstra 算法。使用 BinaryHeap 和 Reverse 实现一个最小堆，加快遍历速度。

参考：https://oi-wiki.org/graph/shortest-path/#dijkstra

### Day 17:

花了很多时间尝试推导出初始速度的范围，~~但是实际上暴力就可以解决了~~，需要进行裁切，具体裁切的过程和分析请看代码注释。（物理的知识完全没有记住）

的确物理知识有些忘记了，但是题目中的探针实际上在每一个单一方向上并不是匀变速直线运动，所以不能使用匀变速直线运动的规律来确定速度范围。

**考虑x方向上的初始速度范围：**

当x方向上的速度减小到0时，还无法进入x的范围时，该初始速度无论如何无法达到目标范围。
**也就是所在x方向上，对于每一个特定的初始速度，总有一个探针能在x方向上行进的最大距离。**
当这个最大距离小于目标范围时，该初始速度一定无法使探针进入目标区域。
当x方向速度为0的时候，设x方向的初始速度为v，经过每一步速度减少1，那么总共经过v步速度为0。
那么实际上当x方向速度为0时，每一步经过的距离是公差为-1的等差数列。
所以x方向上探针经过的距离为 (v * v + v) / 2，设为s。
那么对于任意的v，假如s小于目标区域的最小的x范围，该速度v无论如何无法达到目标范围。
实际上就是求解 (v * v + v) / 2 >= x_min 这样一个一元二次不等式。
当x方向上的速度大于x的最大范围时，1步之后，探针就超出了x的目标范围。
那么x方向上的最大速度为x的最大范围。

**考虑y方向上的初始速度范围：**

因为初始位置不在区域范围内，所以至少需要1步 才能使探针在y方向的范围内。
无论探针初始速度是向上的还是向下的，那么最后总是需要以y向下的速度进入目标区域，因为目标区域在y=0之下。
**当探针从y=0或者经过y=0向下运动时，每一步经过的距离实际上是以公差为1的等差数列。**
所以在y向下的方向上，实际上不存在探针能探索的最大距离，与x方向不同。
那么即使当y=0的时候，x方向的速度为0，且x在目标范围内，探针一定能进入目标区域。
所以对于y方向，探针存在一个最大的同向、反向速度，但是不存在最小的同向、反向速度。

事实上对于一个特定的x方向的初始速度，存在一个时间范围，在这个时间范围内，探针能保证探针的x位置是位于目标范围内
那么理论上就可以根据这个时间范围和y的目标范围，推论出y的初始速度范围，**需要增加运算复杂度，在此可以不看**。

考虑初始时y向下的大最大初始速度，也就是不论方向的最小初始速度
设y方向的范围为y1..y2，且y1 < y2, y2 < 0
那么当y方向的速度为 y1 - 1 的时候，只需要1步，探针就已经超出了y方向的范围
所以y方向的最小初始速度(同方向的最大速度)是 y1

考虑y方向的最大初始速度，这个时候探针应当是做向上的抛物线运动
那么探针会两次经过 y = 0 的位置
而第二次经过时探针y方向的速度应当与初始y方向的速度大小相同、方向相反，记为 v0'
假设第二次经过 y = 0，之后至少需要1步才能进入目标范围
那么当 v0' > y1 的时候只需要1步探针就超出了y方向的范围
故 y 的最大初始速度(反方向最大速度)应该是 -v0，也就是 -y1

**可以看见实际上无论初始速度是向上还是向下的，最大的初始速度数值是一致的**。


### Day 18:

今天的题目大致上应该有两种方法解决，一是建立二叉树，二是对字符串进行处理

利用二叉树方法时，我没想到如何解决寻找附近节点的方法，花了一下午的时间，于是我决定简单的处理字符串的方式解决。

字符串的方式比较清晰，只有在细节上需要想的特别清楚。

现在想起来字符串的方法应该就是树的层次遍历（广度优先）。但是我对利用 Rust 来编写二叉树还是有很多的不理解，涉及到对子节点的修改时我就蒙圈了，需要加强这个方面的学习，如果明天还有时间就继续把建立二叉树的方法也完成吧。

**文件说明**：

aoc18: 二叉树方法，失败中。

aoc18_with_string: 字符串方法，成功通过part1和part2，并且含有相应函数的测试。

参考：https://www.reddit.com/r/adventofcode/comments/rizw2c/2021_day_18_solutions

### Day19:

这个题目其实并不难，但是有好几个关键的部分，第一个部分是，坐标系的转动是如何的，第二个部分是如何确定两个scanner中的beacon是重叠的。

第一个部分，我假定对于每一个坐标位置，可能是x y z中的一种，然后它的符号可能是+、-中的一个，根据这样得出总共有48种可能性，但是题目中说明实际上只存在24种可能。我猜测是和旋转有关，48种可能性种应该有大量的情况是不可能发生的。我没有花大量的时间在确定具体是哪24种，我就按照48种进行了计算。我在Reddit上找到一个关于旋转的说明，大意是有一半的坐标系是不存在与真实世界的，参见：
> https://www.reddit.com/r/adventofcode/comments/rjpf7f/comment/hp7tpyf/?utm_source=share&utm_medium=web2x&context=3
> https://zh.wikipedia.org/wiki/%E5%8F%B3%E6%89%8B%E5%AE%9A%E5%89%87

第二个部分，理论上对于一个已知的scanner的所有beacon坐标，和未知的scanner的一种旋转方式的坐标，从这两个集合中，进行选择组合，对每一个组合进行差值计算，得到一个新的坐标，利用HashMap统计每个新的坐标的出现次数，假如其中一个坐标出现次数超过12，那么这个坐标就是未知scanner的坐标。在这里我还是偷懒了，在选择组合的时候，我进行了全组合，也就是说，已知scanner中的每一个beacon都和未知scanner的一种选择的每一个beacon进行了差值计算，在这个基础上进行统计。我的方法严格上是不正确的，假设已知scanner有25个beacon，而未知sacnner的一种旋转方式也有25个beacon，那么理论上已知scanner的beacon对应未知scanner的beacon的选择组合，总共有 25！种组合方式，这还仅仅是一种旋转方式，然后对每一种组合方式分别进行统计。而我的方法实际上对于一种旋转方式只存在一种组合，组合的数量是25*25。虽然最后取得了正确的答案，但是实际上应该还是侥幸了。

关于这一点，我看了很多的Reddit上的解答，大部分的人都有一个假定，假如beacon A、B都出现在 scanner X 和 Y 中，那么A、B的顺序应该在两个scanner 中是一致的，也就是说对于这个假设的情况下是不需要进行组合测试的。

第二个部分我看到Reddit中有更加好的解决办法，只能留待后续继续实现了。参考 https://www.reddit.com/r/adventofcode/comments/rjpf7f/comment/hp8btm1/?utm_source=share&utm_medium=web2x&context=3

参考：https://www.reddit.com/r/adventofcode/comments/rjpf7f/2021_day_19_solutions

### Day 20:

对于给定的输入，实际上每一次图像增强之后，无限区域的像素会进行翻转，理论上这个条件下是无法给出每一次图像中所有亮着的像素点的数量的。但是题目求解的都是偶数次的情况，在偶数次时亮起的像素点是一个固定的数量，是可以进行求解的。

问题在于每次像素增强，输入图像一周的两个宽度的所有像素点都会受到影响，假如在一次像素增强中，所有周围的像素都没有被更新，那么理论上是不需要扩展图像的。假如不关心这一点，每次都将图像的一边增加2，那么50次之后，图像的大小将是200*200。而每次增强实际上是需要遍历其中每一个点的，那么就会导致运行时间极大的增加。

当前我对这个问题的优化是，先增加边界，当每一次增强结束时，再判断处于边界内的点是否完全都是dark。因为涉及到存在无限像素点翻转的情况，所以只在偶数次对本次和上一次边界内的点进行判断，也就是假如边界内的点全是dark，而偶数次时无限的像素点都是dark，所以实际上边界和无限是一样的，所以缩减边界。

在Debug下的运行时间从10s减少到5s。虽然有时间上的提升，但是很明显我这样的方法是浪费了很多的效率，也就是先扩展再缩减，很明显存在问题。需要思考更加合适的优化形式。

**将每次边界增加控制为1的时候，运行结果不变，不需要额外优化，时间既是5s。我找了一些网上的其他解决答案，测试了一下并没有发现比我自己的解决办法快多少。为什么只需要加1？因为我最初的理解有误，实际上只有周围的一个会收到影响，也就是3x3矩阵的中间会收到边上值的影响，这个时候边上和中间实际上只有差1。对于对角的值来说，它们之间互相不存在影响。所以每次边界只需要扩大1即可。在这种情况下，就不需要进行判断了，判断的成本较高，增加了代码的复杂度。**

### Day 21:

第二个部分，每投掷一次骰子会生成3个宇宙，每一个宇宙骰子的结果分别是1、2、3。那么对于一个玩家投掷3次骰子，结束之后会有27个宇宙。如果直接按照这个思路进行实现，程序运行的极其慢。分析这27个宇宙可以发现实际上最终只有七种不同的结果，从3到9，每种结果对应有一个次数，也就是这个结果出现的次数。假设结果为6的宇宙7个，假设第一次玩家1投掷出了6，那么对于玩家2来说，无论玩家2投掷出什么，玩家2的投掷结果都已经出现了7次（7个宇宙中，玩家1都投掷出了6），以此类推，每次都累积当前玩家的出现次数，在取胜时统计结果即可。

看了Reddit上的一些解法，他们利用HashMap进行cached，减少计算的次数。参考：https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/21.rs

我自己参考实现了一下，我的理解是这个缓存从最底下开始进行，对于一次投掷的所有情况进行考虑，统计在这种情况下，各赢多少并累加。然后再把这个数字和这个投掷出现的次数进行相乘，也就是假如上一次投掷的结果是现在这样的，会有多少赢家。每次把当下的位置和当前的得分以及这个位置和得分下输赢情况存入缓存，那么下次再遇到直接取用即可。逐渐从投掷一次就赢的情况计算至输入时的情况，最后输入的得分就是两个玩家赢的次数。

投掷的时候不需要对两个玩家进行循环，实际上每次只要对一个位置的玩家进行投掷移动处理，然后把两个玩家交换位置即可，减少需要记录的内容。


### Day 22:

应该是到目前为止我觉得最难的题目了，这道题目最直观的方法就是暴力，遍历所有可能的点，然后根据每一步取得最终的状态，但是很明显第一部分都足够的慢了，于是第二个部分肯定是不能用这种方法了。

其实第二个方法也不难想到，那就是把每一行的输入都看作一个长方体，这个长方体存在四个属性，长方体里的小立方体的状态、x、y和z的范围。长方体的体积就是立方体的数量。

考虑体积为 a 是长方体 A 和体积为 b 的长方体 B，假如A和B存在重叠，那么重叠的部分应该也是一个长方体，设重叠部分的长方体为体积为c的长方体 C。对于 A 和 B 存在以下四种情况：

1. A 和 B 的状态都是 on，那么A和B中总共包含的立方体数量就是，a + b - c
2. A 是 on，B 是 off，那么A和B中总共包含的立方体数量就是，a - c
3. A 是 off，B 是 on，那么A和B中总共包含的立方体数量就是，b
4. A 是 off，B 是 off，那么A和B中总共包含的立方体数量就是 0

看起来很简单，但是实际上存在其他的问题。对于题目的输入，初始时不存在长方体，那么对于输入中的第一条为on的长方体应该就是最初的长方体。接下来这个初始长方体会进行下一步，取得下一步输入的立方体，无论这个输入的长方体的状态为何，当这一步结束的时候，输出的已经不再是长方体了，而是多个长方体的组合。而这个长方体的组合，又要接受新的输入，所以简单计算每一次输入之后体积的变化是不够的，因为后续还需要对之前的长方体进行变换。

以下是这个部分的 Rust 代码：

```Rust
let mut stack: Vec<Cuboid> = vec![]; // 初始化空栈，用来存储每次变化之后所有的长方体
    for next_cuboid in &cuboids[..] {
        // 遍历每一次变化的长方体
        let mut new_stack = vec![]; // 建立新栈，防止在后续遍历对栈的直接修改，导致逻辑错误
        for cuboid in &stack {
            // 循环遍历栈中的长方体
            new_stack.push(cuboid.clone()); // 直接在新栈中存入当前的长方体
            if let Some(mut sub_cuboid) = cuboid.sub_cuboid(next_cuboid) {
                // 计算当前长方体和输入长方体的重叠区域
                // 防止累加两次重叠和减去两次重叠
                if cuboid.state == next_cuboid.state {
                    // 假如当前长方体和输入长方体的状态一致，重叠长方体的状态应该取反
                    sub_cuboid.state = !next_cuboid.state;
                } else {
                    // 状态不一致时，重叠区域的状态应该和输入长方体的状态一致
                    sub_cuboid.state = next_cuboid.state;
                }
                new_stack.push(sub_cuboid); // 把重叠区域的长方体放入栈中
            }
        }
        if next_cuboid.state {
            // 假如输入的长方体状态为打开，那么直接把输入推入栈中即可
            new_stack.push(next_cuboid.clone());
        }
        stack = new_stack; // 更新栈
    }
```

**具体说明：**

最初的栈中没有任何的长方体，当遇到输入的长方体为 on 的时候，将输入长方体推入栈。当栈中存在长方体时，需要进一步考虑。

考虑当前栈中的长方体 A 和输入长方体 B，假如A和B存在重叠，那么重叠的部分应该也是一个长方体，设重叠部分的长方体为长方体 C。对于 A 和 B 存在以下四种情况：

1. A 和 B 的状态都是 on，**长方体 A 和 B 都会被推入栈中**，但是这个时候，A 和 B 的重叠区域就被重复计算了两次，所以向栈中**推入状态为 off 的 长方体 C**。
2. A 是 on，B 是 off，**长方体 A 会被推入栈中**，但是这个时候，A 和 B 的重叠区域就被重复计算了，所以向栈中**推入状态为 off 的 长方体 C**，表示重叠区域 C 是需要被减去的。
3. A 是 off，B 是 on，**长方体 B 会被推入栈中**，因为是新的输入覆盖老的输入，所以向栈中**推入状态为 on 的 长方体 C**，表示重叠区域 C 是需要被加上的。
4. A 是 off，B 是 off，**长方体 A 和 B 会不被推入栈中**，栈中的所状态为 off 的长方体都是由重叠产生的，那么 A 和 B 的重叠区 C 已经在重叠区域中表示了。所以向栈中**推入状态为 on 的 长方体 C**，表示新的重叠区域 C 是需要被加上的，防止重复减去重叠区域。

最后只需要对栈中所有的长方体进行体积计算，加上状态为 on 的长方体体积，减去状态为 off 的长方体体积，最后的结果就是所有步骤之后立方体的数量。

**计算重叠区域的长方体**：这个部分是我觉得最难的地方，因为我想的太复杂了，没有想着从一条边的情况开始考虑，实际上长方体就是长方形的累积，长方形就是线段的累积，线段的累积是好分分析的，那么只需要从线段开始，就可以轻易得出重叠部分。

**考虑一条线段的重叠情况**：假如一个线段的起点落在另一个线段中，那么这两个线段就存在重叠区域。具体计算和说明见以下 Rust 代码：

```Rust
fn sub_edge((a, b): (i64, i64), (low, high): (i64, i64)) -> Option<(i64, i64)> {
    if a > high {
        // 假如一条线段的最小端大于另一条线段的最大端，则不存在重叠区域
        return None;
    }
    if b < low {
        // 假如一条线段的最大端小于另一条线段的最小端，则不存在重叠区域
        return None;
    }
    let low = low.max(a); // 重叠线段的最小端是，两条线段最小端中较大的那个
    let high = high.min(b); // 重叠线段的最大端是，两条线段最大端中较小的那个
    Some((low, high))
}
```

**考虑长方形的重叠情况**：假如两个长方体的一边都不存在重叠区域，那么两个长方体肯定不会重叠。而重叠区域的计算，应该就是两条边分别计算重叠边，这两条重叠边的区域就是重叠区域。

**考虑长方体的重叠情况**：根据长方形的情况类推即可，代码见如下：

```Rust
fn sub_cuboid(&self, other: &Cuboid) -> Option<Cuboid> {
    let x = Cuboid::sub_edge(self.x, other.x)?;
    let y = Cuboid::sub_edge(self.y, other.y)?;
    let z = Cuboid::sub_edge(self.z, other.z)?;
    Some(Cuboid {
        state: self.state,
        x,
        y,
        z,
    })
}
```

**优化**：
在计算的过程中，我利用栈来进行统计，实际上可以用HashMap来加快统计，代码见如下：

```Rust
fn calc_volume_with_hashmap(cuboids: &[Cuboid]) -> i64 {
    let mut counters: HashMap<Cuboid, i64> = HashMap::new(); // 初始化空表，用来存储每次变化之后所有的长方体和长方体出现的次数
    for next_cuboid in &cuboids[..] {
        let mut new_counters = counters.clone(); // 复制为新的HashMap，防止遍历HashMap的过程中对其进行修改，导致逻辑错误
        for (cuboid, count) in counters {
            // 遍历上一次的长方体
            if let Some(sub_cuboid) = cuboid.sub_cuboid(next_cuboid) {
                // 计算重叠区域
                // 重叠区域的次数为减去当前长方体的次数
                // 类似于利用栈实现的时候，新的长方体的状态为当前长方体的取反，具体见栈的实现的说明
                *new_counters.entry(sub_cuboid).or_insert(0) -= count;
            }
        }
        if next_cuboid.state {
            // 假如输入长方体状态为 on， 直接将表中的长方体的值加一，即出现次数加1
            *new_counters.entry(next_cuboid.clone()).or_insert(0) += 1;
        }
        counters = new_counters;
    }
    // 计算总体积的时候，要将长方体的体积乘上长方体出现的次数
    counters.iter().map(|(c, w)| c.volume() * w).sum()
}
```

**结果**

```shell
there is 420 steps
Part1: ther is 648681 cubes are on the initialization procedure region
Part 1 took 8.63525ms to computer
Part2: there is 1302784472088899 cubes
Part 2 with stack took 790.735791ms to computer
Part2: there is 1302784472088899 cubes
Part 2 with HashMap took 264.527791ms to computer
cargo run < input/input.txt  1.28s user 0.37s system 75% cpu 2.177 total
```

可以看见 HashMap 的时间快了很多，理论上还能进行优化，但是这些优化都是极小的，所以就不再引入了。

参考：
> https://www.reddit.com/r/adventofcode/comments/rlxhmg/2021_day_22_solutions/
> https://github.com/satylogin/aoc/blob/main/archive-2021/day_22.rs