# GoldMarket

simulation of foreign exchange transactions in an overhead world

---

## 这是什么

假设在某个世界中存在这样一个允许自由交易的市场，其中存在Pa、Pb两类不同的交易方。除此之外，还有G部门负责产出并供应黄金。

---

## G

G是黄金供应部门。已知：

1. G部门每天可以稳定产出一定量的黄金（以重量计）。记理想状态下，G每天产出的黄金质量为g，则实际G每日产出的黄金质量为：

$$
g_{ideal} = (1-\eta)g, \{eta\mid\eta\in[-1,+\infty),\eta\in\R\}
$$

## Pa

Pa从G处获取黄金。已知：

1. 每天，Pa都可以从G处获取当日产出的全部黄金。
2. Pa可以选择将黄金投放市场交易。没有投放的部分则会留在Pa处作为库存。投放的黄金量不能超过Pa的库存。
3. Pa也可以选择从市场回购黄金。
4. 黄金交易使用货币R

---

## Pb

Pb是
