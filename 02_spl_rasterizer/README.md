简单的光栅化三角形渲染器，模仿了老式固定管线：

> Vertex Shader -> Rasterizer -> Depth Tester -> Pixel Shader

随便摆了两个三角形和两个点光源，效果图如下：

![spl_raster](./../docs/pics/02_spl_raster.png)

顺带一提，大概是因为我不熟悉Rust，Screen Buffer写了好长，很是难受，随着以后的学习慢慢改吧。
