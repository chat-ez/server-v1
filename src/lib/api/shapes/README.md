This shapes directory is intended for use across multiple packages such that 
the structures can easily be reused in both client and server code. Handlers
define their input models via {RequestMethod}{ShapeName}{Request/Response}

Responses `impl IntoResponse` in order to automatically generate responses given
pre-defined fields in the shape and the client can expect the same response shape
when serializing the incoming response.
