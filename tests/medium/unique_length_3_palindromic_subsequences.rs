// 1930. Unique Length-3 Palindromic Subsequences
// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut count = 0;

        // For each possible outer character (a-z)
        for ch in b'a'..=b'z' {
            // Find first and last occurrence of this character
            let first = bytes.iter().position(|&b| b == ch);
            let last = bytes.iter().rposition(|&b| b == ch);

            // If character appears at least twice
            if let (Some(first_idx), Some(last_idx)) = (first, last) {
                if first_idx < last_idx {
                    // Count unique characters between first and last occurrence
                    let mut middle_chars = HashSet::new();
                    for i in (first_idx + 1)..last_idx {
                        middle_chars.insert(bytes[i]);
                    }
                    count += middle_chars.len() as i32;
                }
            }
        }

        count
    }

    // Time Limit Exceeded
    pub fn _count_palindromic_subsequence(s: String) -> i32 {
        println!("s: {:?}", &s);

        let mut set: HashSet<String> = HashSet::new();
        let chars = s.chars().into_iter().collect::<Vec<_>>();
        let len = s.len();

        for i in 0..len - 2 {
            let l = chars[i];
            for j in i + 1..len - 1 {
                let m = chars[j];
                if let Some(_) = chars[j + 1..].iter().find(|&c| *c == l) {
                    set.insert(format!("{l}{m}{l}"));
                }
            }
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::unique_length_3_palindromic_subsequences::Solution;

    #[test]
    fn test_count_palindromic_subsequence_1() {
        let s = "aabca".to_string();
        assert_eq!(3, Solution::count_palindromic_subsequence(s));
    }

    #[test]
    fn test_count_palindromic_subsequence_2() {
        let s = "adc".to_string();
        assert_eq!(0, Solution::count_palindromic_subsequence(s));
    }

    #[test]
    fn test_count_palindromic_subsequence_3() {
        let s = "bbcbaba".to_string();
        assert_eq!(4, Solution::count_palindromic_subsequence(s));
    }

    #[test]
    fn test_count_palindromic_subsequence_4() {
        let s = "tnapzbjeznakaxowyeqefiwxpoxswedvabnyyuihjenxmpzxyyokldoijgvekjx\
        xvxsvcnrinonkofilfyllcacophzuusnbyhpxoqtnbhezbvwtnejsxcyxsbffqaxfryagvvzzvjvbdg\
        rwkowqfwthrkkhxvmpqkslrfqcxmeiygryknhocdvwyomdzmcfvetugpldpbytefioyiyxjfqkdhbef\
        rlwbgnodzbrknqeyjdcgjovtdfqobqxyqhqltrpizlxdnjdqezzwjaudlsofkvjykktlkjiksffefwr\
        qxotkqfdcqhvuqbfoejnelskrhtoekufkmwdyiyptwrucawbwixfdfvjxpvmshcoxdknqeomzrxzrdl\
        tdjjearvypexzyoxzxbdhkdzurzisycpuaxkewehnxmxbghllbttlcodzqtajxjarsiuwukiysomgxx\
        crincvucbjeuuwaauwqeqwlwucdsftccyaugbcinmfzpehrwbeaefdozssbeizeqppnhlvtnrzgtbbi\
        tqvctsatcpxjrrbgvltmmtizepbjmmywzmxldtkadizqkaurepeckdiakhyofmslyezbybimhgyukai\
        eqrlddhsuwjzblkfigwixzuopifdiqzfpgvmmanctqcmpxygluuhdwbgqivmdhizpnvqypdttfuhukv\
        agdaqtcmxdqoptchhsledydwojisoqqkivahoowbsdbbdnygtziktmonqkevvlcxggqobvfbfgdwnit\
        ratmrsobpfpaspjxefnxthonefpiigdhapqnkebutwfotvrxvjbqowgnifeimsslktxmpuryccaabah\
        alsycjqztjhqhaddlyzxdqyuwyecslxshaobrfkjalmeaxrywssfgrzervzbowosusthcjvojxsoiqk\
        pjztdrxrkycgdyqlorxixzdiqtwozuvkvwzonogoytrumivjynvfjktcqlvxeauncdbygkvocvaadeu\
        bgykjkshkflpmpbmaiaswjocrqjzmabmxmocssxltgzaiuwfkadwuylnmskyqbpsmxhehicevfmauux\
        ubeirvflsxcdtyoljsansqcmhokbgyjkxxwdeorlzjgbuekogddsabucrpawdxwuemmckqouddnwbnz\
        ogtvjfgblwuarykpwtdpuxqtephulabcxdqjkmuxhniiynkwrsypromcminqgghhnixxojhvnkglrtx\
        duhrjoiupeswqfuveqmgmhondpgzvzaeyylzphofblzpmkdejubwhoguyrfxoaiqhlfqogovdoifuvf\
        qyctzhmcvfleefcgippkfccsltjpjohwmunimqvpjspsjufikghgpmaygslukijmsxcgfjzmewqpvvz\
        jtcxcsbcylhwhxfingcbruyyeoyxgvfsbcvedmtsdygawtnanbdbedwvohgecgqtbjalurkoknzyjps\
        ttnknmufpjhpkjjifalqdwutqhgaflmmmwmkbyxxhbmkqlpjnmywmjfxezdshafnakulqhbndlmtaus\
        obzyjrlsgdxcjkbhyfokvdogxabwgmbmzymclxqzwygoxvirnrvpruugvlhyhzzbpugvovfymdcojtk\
        icemybixlvemaydgwmypkwhpeeijccuaomczvqzaedzjrvjobxibpzpveekzcpqnvhbpgqscprawrsv\
        jnhskugziigdecpdcldburpsldqyopyfmaeebawgemrksidizkixosauykmzdwgpyxskuqsujamvnfi\
        rtnnxzpplyulsootbikqumwyxpgiplhqbfxggnbylklyuhvoknyuqhpbzfrhdgzoiofrcqyadtgrrkx\
        fecyioobzwwaguuyzcldtumdrhcujkeysxqztzeinrfztjiktcnvfequtbzfrxmukqfblajzyqkvzwc\
        vpoddmzsiqaviivhwbhzdgjdikbnbihaziswypwjcgixefughvoowvoprhohxtbxkrgbbypcmmjogpo\
        uceqowqafseywnzanhuueobfrfnikpxhgwdkqmzkegdeqdrdbzttrdfyoqsvacfdsuwikrrlfqfjwwr\
        mylgleuvtnfchhetmtblhffwvzhmlduqjzoqdgjgcwnikbycxggyywoyxqzayirooafchtgupqryojj\
        qxiwjtnmlorcxgwfkzbrzldtfcgxkzqxxcypcgptygmhetsymzuxcnfoasytoetzorfvgbevfhblqas\
        sjttvlrvejeaevkcebklsesswnxpzpilvprflevlipyuvfhumcpudbtnwokqsvbewugbwobocncgovb\
        mvduscdlkqctlrziaaaravaitvpceycuojutpszirunhheodrohntdvhqbjblapaggiijnnkogdhfir\
        jvujbsqbfkwoquaqqyyebgbdkvdonaqgweymynjtnbgebsqheorskegksjsnfiucwvhzkjttwbkfrir\
        hyxacjkhhovnxutjonezizmvbrarbwgxftjvgiskfdccpnsuopfpqffkifwwsyxpntikmnkowxaevza\
        cucdpygjmsghzdskaqfjudbibdnuhofkmvptkesypnblikcyswhnktvquimklubsdduabfznfkwcwae\
        litcefxjvneomwloxqzvhzdhouzuigwxtnkfqpdcjravnzpbhoyslgukpoykqvwrvqttrtwcndupqod\
        ohuxouwyxcvufuuloybqaoeqlkxqefozfpirxqxnroavhaikrzxolauuktodrcooiqyfovcvwtuiiyd\
        vukdpuwhxwvetarnpxkzdzkfcfpvvayjbqcqbtbkeqfplkdsfhamlhesldkwrpiocwsvxyzoybuayrg\
        zflcdyjyfxkmymfzxrcejfkvzimozjelxohpsepbwhbvlhblrvknyxtmnxbvqwxpvcfnzeotmtpglvw\
        owweriaftyaupslrzlskygkudujaocveqiezrraizyrkdbqyboiklpbpuglphoaexaiapsrozprnhdw\
        itnqqhlolvxsljexkkftraqhuzbqzmnjdqxnseviomtunraqrujswrwtksilarexetwgenlkdvpegyr\
        twuvfyoxockgrxetsyeykzeyfsiprcwycsjaqnyyoaimrvffqdrnpycmmbnspgavfhiliicsjxvmvng\
        lwbygwsqcyuajlqydpmbtdmhdvnwjedivvlbiunullhemiitroipzuispedyxzvlidwwbujfqgydwwv\
        uwbaiafqoechwnjpzmmwzflqlyqdajntejfynueovtebfdvahcjacggesmaemncluxmitjqmjwyiqnu\
        rlxoskoeldmbrfusyrzyyieooptwyturpqgogeqsgpxfoikmqtwfvhyoggvupselvwleebbgxmnmcmx\
        ipoxgdvtcukuwoewgopyxnnojftsjneqkbvtztfhhxqvwocbvefsxwadpuspjahmqtfpfakmfpczdbd\
        dfdrslmrbptcckdedslsrytyfqsaolinrlwvucthchurqdiarzdhdvpsjzsvbtrufrpdsmaczdhljij\
        hyfayxuazykjegmbpbszlwafdhqxigwvrqfshgdqxkmxqjxiblrzjlegmcvglcotunbaxuocktcenon\
        vrldaszsmfblgmfcbwjnhhuesaoennjllgadafrwfkvxlglkvcpnvsmazkliiptubprdgarpoybstwq\
        hagzbubpnvhvvykpzqqfprvjohkjcfhbmggeietzrppxzlasxelzramnvoecberauqcyyyugrtopxul\
        dczztnqzynvwzyxhdtlwjnyolnlaevgbmmcvvjcpcqcexrpuplxdskyrwrbbjsguuljbxqehxroijcy\
        pwrqeqxyshpjggpalibaocdzjtvkodnoufnnnpwdgmztlolqbhfvptsjtpvzwscfsxizbgocivaexhh\
        ddciafhaokbtmxtudgntqoyswmocchnzuvqxqyoqnugzkqlyjixcigircpsrbrmjsjsjbcrbvnuufik\
        sdngjbmbpqhdnpehjjwutolmbugauiljtixouiifuuadwjuwuweksbkipfbsreeehzulwsmktpkcesc\
        jznynnkbtzamnkszdwepyluaulzkgevbsxbnftmfipktbozqjgnidotnwjtybrlhjqidfwcyesluptz\
        rqvclutghqtmawojebmsrbwpkgbiqapufsndqfwwawwutnskmhxviegxyglvfnjbpfsoarturgpdlnr\
        wtuolpnexrbzipdmhldjacsptjggxvqpegoczzrmkvyoznnbpyedvafxarklcxnqroumkachjuhxgoy\
        qqcnbmktwqfiwpvuafuavkejvyyplbarlzrkcaqxhccvzfhvocpxpumnautlohdnoqepunoonqdnrqu\
        bauljvoxapdydwtjwqpveazoayxjdbliiyllsrdhpucrvohppyledyiplvwxgtgdvjhmviudvdvjvsa\
        tbkyxkylfdwmeeoisuacnyversgdkkyyujmbjosycozpvxcvcecwtfvubofabliaygszcnuoihvdomp\
        owkdgytktyundmuvcxsbgbjopfemhzuslnwgwdnkhoblfozzxynozifxuqwkdbillhbjqjjkfevpoit\
        cmssrpxcccuyczaihyrbseasjgvysceacdliuowbekvxccfsewzrrgqqahymtsvmoyspjaftbakyqzu\
        arpajywlnstboqxyzipxyjtgjodjbbzopqklxvbosmbsgmgrsyrohpzjfamjweifqvbfjgxecnebeys\
        duiwbtabxkrjatoxngdxckfdyhryyltzozlwwcpplebygrwuxdjncmmapalciesoelycusejxunxsro\
        lunfjocpitrmscobbzignvpfojynpxuhyibcnetebopnqvfreghmhfdtnckyjpfadqaafbysfdsmwdw\
        gsbudcibgfrviugvljxcndbaepkwrtmjpwetndugvtvbkeoufdmnjojvvxltxjsycnkjhhtgrllzrpe\
        fkaqjldedwdhdfkdvirsgdrwqynmzyqtydgivcprgfigdzzyzejbnfgfvvgcvwnkjsjmsxogilpxugu\
        xqyjgjmwkypnzbfhcijnxoyyxhcitpiyokvewglzdjszgkyhzyqpgxyvhiisbofufyranrdyvxddjoc\
        yhejzhxlxugvqrbmqeqvanuwvfllpkiemcxqbtsvvybyjqrbadwediirydahzrdxtnqczxffqjeuesp\
        eswuiszjxrfgequlsenpajmsxvufflnzmcbzbepudnewjcmecqyzfzrqecwrvlsgwumitdfzfrdyxnq\
        thzizwosrrweykjvqzfrxgxumubqnhrmcgmiwyuqiarfkkburfvmjbsubiouyrjlsboxilaycspfioj\
        emadgleaqqcschyotlyknqkonfemmcpudqraysznmojvyyrxsiccuieyruqedbdbvumjzolxmsakoki\
        rlkopyxcvkjrawakjsmatzoqzlcqqdsffspsejeekfhzwpxqooikxklhigxythgbxpkyptgswhlfwwg\
        nrrodbikpmyzrnunwhgcaxodrcmbixnfqujmpwdgbhlhydpovdtjdctlwwnpepqvzoluxpygjhgikcf\
        ojxheverdwflekraglpnsukvvauzqsxohtyducrmxbgzhvbfuvyevphketsttfdzqgddkzfwffgmbrr\
        llyawdmptixbhjbyegpuveahxubolyuvuxaleeuhwbvsxgwmucxqrigcupwybjonxerqiwbepuprkth\
        gntymqlubedcbnzbszoutbxzbwwmsuypjpqgjptxccexytgoeawdcerrgwjyxtaofzhlxannnpihaew\
        pjnjpwermqxkqfypigexmuejdkxjcvdroydhbyytfditsklmsekvsrybjignqocfcrtbhnwsevcwcvu\
        jhrwsnwonzhatciobihdvaueyxmqmvnuyqhwtudldhepsldbehbhpnuctyxowwcwlvkpzhrcescobwk\
        areaaytofkwaqiegngsyoekveutblnlvtyweoxwnxrwfvktnqwzxttelgozczeruffpconzjetsbmst\
        yaymkqkqjpaxncrguqmlklnczkwqohrerzvdkjykjewocifqpxeiprqalhdwagmtkpzblqmoophrgjq\
        okbnuuwcmhlyikpvcjungiuequwievzedlkhpbvengnuawoerbmrogpmvtcjzgdyuxbfrstkpwpqazj\
        hotelovzlonmsadmkqactlxdgxhzdpbmryoyvjhbdudxplmileyepifqrlammkorippicvgfavzhsfe\
        vxbtkryybsmyqqmeunogjiroephrjsikzijcbibwbiclhmtqjdlnttculjhuofhxfimnfdfcurgqxzl\
        amnexbummavlibelhirysmuncoclmhwmtljevpbifkpoxqlzipourcnompngysesoknxehhyuhtxrsp\
        lowvxisjnytvbdlptpxvvvhoqxlcaiphzyvuufmwnkncsqfntwccgzmohxoxaurwvovvlpjdwypfucv\
        inttmttrfratdgpaynwzhqidsbbnvuwsbiwhsfbisumjkxcrnpofgqsqozzvljylgqwvgmejwchkrbd\
        ngmlwoktjaxhnjgbskaczmyqfnvxkgzzarodzadkkyncqrlnppuyoobmgtxwqraenthplhiycsvtnxd\
        mlxazuyzutvvutahfnnbmoltzfgktlvcspqcyfvwzeddgebgrewxpsjmatbbrrvhoejuvqbmqcqdtom\
        ptmtzwofecmhdhgxqfkcvbfjlrfmdkseydapugxkdofypampacgiuzalpctalecojbnwrmeavixutuz\
        cevzmaggbzvthuxedqaifptdtysphvumeqfeqjcxmcmvsazdyvjocacocynxcuokthpnkmlwzrdmzxn\
        hlatlrzzcahwdlbkttytyfpsdfifonhybefzkhesabetmydaxaffbanghskgcanouzdduaywjtqvdmy\
        tpnustqkothzyywomggbxvznrgdcvyccwirojgntzizegwajigkaiypgpswldbbdwsvsocnxqnptzdx\
        dnnbgidfxqfiyfzahecuyoioryquziarxcameblaconlastezdtyljdrgnjmltfvxxtiszhedlzwhva\
        qiamtqftmdgokmkeyttdlykznwskjskddzyqlzwlwhqmvrpqjazcnnmhdgwvhhktjdcculgtjjoezrh\
        hvtcsmfonmargfrunwusnhunzkwawvzeewirhuewdkitnaymcswkzfbsclvqxbchyfxmdcrtpmqkucq\
        kytyrqtbncqvqvijadflfnsaxorvnadmtsmycarcqzexrwpzgmdfiflkwxauorsjjwnaqrhudwzevka\
        zptowgraurjlrgwxjtwditvxvkayfjzyqtxewssqwphwnfaxtpvvifdjbmjuokhsgrywgocivwhmgzm\
        vplyigitxcoblmudptmcjewqtvmfjdultzukmdjlafvmjutmztmsfmhfaelvvnileboiuknzxenhbkg\
        aqjfmbvfiszkkdpfnsvpkragxnkresuugjanvumptyiwlgaeriltnwmiglcsavndefkvlfbeydctvvq\
        mpzceuewjhvkoaeclpfweaekyooczgrgvbowlwtxucyluxryhfevexjfcvehlzxafottibwqpzisfpd\
        szmocxdkjbzoldlkkiewohzpgtpasnohatztwabascyvpeuxuywgagoafxjffaejbvxuizcxfiisxfk\
        noosffufewkelanjtnqdvutcwbutkxppydgmfmijexjpwqoxizcdkbfmgkywhirnxgcxsxlltaoetng\
        whesbrwjihsdpcgvnlxldmzjtwdcosguiyuptzhomlqvffgepojjqbisryhwinutnvoziluduankdpe\
        hfuwtcpeeejtnjuyktzloetwqcyughhopuprkvldkgvivzjfbvqagbtcfjfqdhmcpjabzydmakkytvb\
        zwkfnyaofehcvrwrfwfkayqhcldfcjxhxfemxnmkxawfghqbhlbiinlzvmqehewhfhucmfiuunkgoze\
        wujfvdobrentgqzainixmyekhipaupfnglhwqthlynhlkjolqwwgkmrkgovzbrwcjnpablyevanisqv\
        khgkwaxfgglnfkqraksxdyeekwxqupulqlftrvbmfkhv"
            .to_string();
        assert_eq!(676, Solution::count_palindromic_subsequence(s));
    }
}
