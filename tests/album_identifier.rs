use hexendrum::library::album_identifier;

#[test]
fn album_identifier_normalizes_common_soundtrack_variants() {
    let base = album_identifier(Some("Mick Gordon"), "DOOM (2016)");
    let ost = album_identifier(Some("Mick Gordon"), "Doom Original Soundtrack");
    let official = album_identifier(Some("Mick Gordon"), "Doom: Original Game Soundtrack");

    assert_eq!(
        base, ost,
        "soundtrack variants should resolve to same album id"
    );
    assert_eq!(
        base, official,
        "game soundtrack variants should resolve to same album id"
    );
}

#[test]
fn album_identifier_ignores_features_in_artist_field() {
    let base = album_identifier(Some("Artist Name"), "Sample Album");
    let featuring = album_identifier(Some("Artist Name feat. Guest Singer"), "Sample Album");
    let with_extra = album_identifier(Some("Artist Name with Guest Singer"), "Sample Album");

    assert_eq!(
        base, featuring,
        "featured guests should not change album id"
    );
    assert_eq!(
        base, with_extra,
        "with-guest markers should not change album id"
    );
}

#[test]
fn album_identifier_keeps_distinct_artists_apart() {
    let queen = album_identifier(Some("Queen"), "Greatest Hits");
    let foo = album_identifier(Some("Foo Fighters"), "Greatest Hits");

    assert_ne!(
        queen, foo,
        "different primary artists should produce distinct album identifiers"
    );
}
