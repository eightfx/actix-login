
# Table of Contents

1.  [これは何？](#org8c841d5)
2.  [使い方](#orgad01265)
    1.  [DBを用意する](#org5184761)
    2.  [.envを作成する](#org5454d43)


<a id="org8c841d5"></a>

# これは何？

actix-webでログインAPIを作りました。


<a id="orgad01265"></a>

# 使い方


<a id="org5184761"></a>

## DBを用意する

適当なDBを用意して、table.sqlのようなユーザーテーブルを作成します。


<a id="org5454d43"></a>

## .envを作成する

.env.exampleを参考にして.envを作成してください。

-   DATABASE<sub>URL</sub>
    これは作成したDBのURLです。
-   RUST<sub>LOG</sub>
    loggerの設定です。開発時はdebugがよいと思います。
-   AUTH<sub>SECRET</sub>
    JWTトークンの秘密鍵です。

