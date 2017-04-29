error_chain! {
    foreign_links {
        Io(::std::io::Error);
        SerdeJsonError(::serde_json::Error);
        CsvError(::csv::Error);
        GeoJsonError(::geojson::Error);
        ParseFloatError(::std::num::ParseFloatError);
    }
}
