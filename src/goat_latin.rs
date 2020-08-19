#[allow(dead_code)]
fn to_goat_latin(s: String) -> String {
    let ret = s
        .into_bytes()
        .split(|&x| x == b' ')
        .zip(1usize..)
        .map(|(x, i)| {
            let mut v = match x[0] {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => x.to_vec(),
                first => [&x[1..], &[first]].concat().to_vec(),
            };
            v.append(&mut vec![b'm', b'a']);
            v.append(&mut vec![b'a'; i]);
            unsafe { String::from_utf8_unchecked(v) }
        })
        .collect::<Vec<String>>();
    ret.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            to_goat_latin("I speak Goat Latin".into()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
        assert_eq!(to_goat_latin("The quick brown fox jumped over the lazy dog".into()), "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa");
        assert_eq!(to_goat_latin("Each word consists of lowercase and uppercase letters only".into()), "Eachmaa ordwmaaa onsistscmaaaa ofmaaaaa owercaselmaaaaaa andmaaaaaaa uppercasemaaaaaaaa etterslmaaaaaaaaa onlymaaaaaaaaaa");
    }
}
