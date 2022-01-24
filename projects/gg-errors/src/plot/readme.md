G2

- 图形属性 Attribute：负责将数据中的变量映射至图形空间。
- 度量 Scale：数据空间到图形属性空间的转换桥梁，每一个图形属性都对应着一个或者多个度量。
- 可视化组件 Component：也可以成为图表辅助元素，用于增强图表的可读性和可理解性，在 G2 中，提供了丰富的可视化组件，包括坐标轴 Axis，图例 Legend，提示信息 Tooltip，图形标记 Annotation，滑动条 Slider 等。
- 分面 Facet：描述了如何将数据分解为各个子集，以及如何对这些子集作图并联合进行展示，主要用于多维数据分析。

GG

- Guide：参考，图形对象间的比较、分类和对齐等，例如图例


```vega 
    Plot(data)
+ 
  Plot(data: dexp_small, aes(x=Sample, y=Expression))
```

data: 感兴趣的变量 (data frame)

aesthetics: x-axis/ y-axis/ color/ fill/ size/ label/ alpha/ shape

geometrics: point/ line/ histogram/ bar/ boxplot

Facets: columns/ rows

Statistics: binning/ smoothing/ descriptive/ inferential

Coordinates: cartesian/ fixed/ polar/ limits

Themes: non-data ink

transition_*() defines how the data should be spread out and how it relates to itself across time.
view_*() defines how the positional scales should change along the animation.
shadow_*() defines how data from other points in time should be presented in the given point in time.
enter_*()/exit_*() defines how new data should appear and how old data should disappear during the course of the animation.
ease_aes() defines how different aesthetics should be eased during transitions.

transition_*() 定义了数据应该如何分布以及它如何在时间上与自身相关。
view_*() 定义了位置比例应该如何随着动画变化。
shadow_*() 定义了如何在给定的时间点显示来自其他时间点的数据。
enter_*()/exit_*() 定义了在动画过程中新数据应该如何出现以及旧数据应该如何消失。
ease_aes() 定义了在过渡期间应如何缓和不同的美学。

- 几何标记 Geometry：即你在图表中实际看到的图形元素，如点、线、多边形等，每个几何标记对象含有多个图形属性，G2 的核心就是建立数据中的一系列变量到图形属性的映射。

- DataFrame：数据， 从数据集中创建变量的数据操作
  - 列表
  - 矩阵
  - 映射
- Coordinate: 坐标，数据是如何映射到图形所在的平面的
  - 笛卡尔坐标
  - 极坐标
  - 螺旋坐标
- Graphic：图形， 在图表中实际看到的图形元素
- Scale：标度，标度转换，例如 log 转换
- Trans：转换，数据变量之间的转换，例如排序