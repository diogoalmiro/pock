module Index exposing (..)

import Html
import Html.Attributes

main : Html.Html ()
main =
    Html.div [] [
        Html.text "Hello, World!",
        Html.a [ Html.Attributes.href "World.html" ] [ Html.text "Hello" ]
    ]