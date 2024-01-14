module World exposing (..)
import Html
import Html.Attributes

main : Html.Html msg
main = Html.a [ Html.Attributes.href "/" ] [ Html.text "Back" ]