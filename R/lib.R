#' extendralloc test
#'
#' @importFrom methods new
#' @useDynLib libextendr_alloc, .registration = TRUE
#'

#' @export Test
Test <- setClass("Test", slots = c( pointer = "externalptr" ) )

#' Create a data frame
#' 
#' @export
setMethod("as.data.frame", "Test", function(x, ...) {
    .Call("wrap__as_data_frame", x@pointer)
} )

#' Pretty-print a description of the Test object
setMethod("show", "Test", function(object) {
    cat("Test\n")
} )

#' Create a new Test
#'
#' @param .Object base object
#' 
#' @return Test object
setMethod("initialize", "Test", function(.Object) {
    d <- .Call("wrap__Test__new")
    # extendr is setting class, but we need to strip it to fit in the slot
    attr(d, "class") <- NULL
    .Object@pointer <- d
    .Object
} )
