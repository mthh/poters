error_chain! {
    foreign_links {
        Io(::std::io::Error);
        JsonDecoder(::rustc_serialize::json::DecoderError);
        JsonEncoder(::rustc_serialize::json::EncoderError);
        CsvError(::csv::Error);
        GeoJsonError(::geojson::Error);
        ParseFloatError(::std::num::ParseFloatError);
    }
}
