use std::collections::HashMap;
use itertools::Itertools;

struct Mapper {
    map:HashMap<String, u32>,
}

impl Mapper {
    fn decode(&self, wires:&str) -> Option<&u32> {
        self.map.get(&wires.chars().sorted().collect::<String>())
    }
}

fn main() {
    let lines = input();

    let mut total = 0;
    for line in lines {
        let mut split = line.split(" | ");
        let input = split.next().unwrap().split_whitespace().collect();
        let output = split.next().unwrap();


        let map = map(HashMap::new(), input);
        
        let mut mul = 1;
        let mut res = 0;
        for n in output.split_whitespace().rev() {
            let decoded = map.decode(n).unwrap();

            res += mul*decoded;
            mul*=10;
        }

        println!("RES: {}", res);

        total += res;
    }

    println!("answer: {}", total)
}

fn map(mut m:HashMap<String,u32>,mut input:Vec<&str>) -> Mapper {
    input.sort_by(|a,b| a.len().cmp(&b.len()));

    let one = input[0];
    let mut carry = vec![one];

    for segment in input {
        let sorted = segment.chars().sorted().collect::<String>();

        let result = match segment.len() {
            2 => 1u32,
            3 => 7u32,
            4 => 4u32,
            7 => 8u32,
            5 => { // 5,2 eller 3
                if contained(&sorted, one) { 
                    3u32
                } else {
                    match get(&m,6) {
                        Some(six) => {
                            if contained(&six, &sorted) {
                                5u32
                            } else {
                                2u32
                            }
                        },
                        None => {
                            carry.push(segment);
                            continue;
                        },
                    }
                }
            }, 
            6 => {  // 0, 9 eller 6
                if contained(&sorted, one) { 
                    match get(&m, 5) {
                        Some(five) => {
                            if contained(&sorted, &five) {
                                9u32
                            } else {
                                0u32
                            }
                        },
                        None => {
                            carry.push(segment);
                            continue;
                        },
                    }
                } else { 
                    6u32 
                }
            },
            _ => { 0u32 },
        };

        m.insert(sorted, result);
    }

    if carry.len() > 1 {
        return map(m,carry);
    }


    Mapper{map:m}
}

fn contained(segment:&str,digit:&str) -> bool {
    for n in digit.chars() {
        if !segment.contains(n) {
            return false; 
        }
    }

    true
}

fn get(map:&HashMap<String,u32>,val:u32) -> Option<String> {
    for (k,v) in map {
        if *v == val {
            return Some(k.to_string())
        }
    }
       
    None
}

// 0, 9, 6 len=6
// 2, 3, 5 len=5

// 7=8, 4=4, 3=7, 2=1
fn input() -> Vec<&'static str> {
    // vec![
// "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
// "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
// "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
// "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
// "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
// "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
// "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
// "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
// "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
// "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
// ] // 26, 61229

    vec![
"gbcead cfgaeb beadf adb egafd fbeac dbfegca fdaceb dbfc bd | dfcb gedaf dcfb bcfdea",
"dgfcb gcdeafb eg egb fcdebg cegf becad dfbcag fbgdea gecdb | dbfcge dacbe bge fgdbca",
"bgdacf ga egab bfeacdg bfaec agc efbcda agfec cfedg abfceg | bega cga febac aebg",
"cbgfad ed ecdgfb dge deabg fgdab geacb gabfced dfae dbgfea | afed dfea de eabgc",
"fdaegb fae acedgfb dgbae fedcb daefb gebadc af bcfgae gafd | egadb afdbe eaf af",
"cfg cegbdf fecgb fg cdageb fbgcad egfd gdcbe eafcb abdgcfe | gdbace fbgce gecfb bdfcge",
"bgcfed aebgdcf gcbd edbcf ecdfag ebc efcdg bc befacg daefb | dfabe edbfc cdebf gbcfea",
"dbfge cfebd dega facbge adbgcfe bfage fgdeba gfdbac gd dgb | bgd eabcfg bcdfega egafdcb",
"ae egca gdeabf gcfba fcdbag dafcbge dbcfe efa ebgcaf fbcae | cdebgfa gecdfba dabcgef bfceag",
"dagcfb fdegacb cbgad bedcg cbgeda cfbeg aefgdc abde ed gde | egdbfca gcbdae gde gecdaf",
"fcdaeb gecaf eabcdfg efcbgd bge cadeb gbad becga cbeadg bg | ebg beg edabfc cbeda",
"ed cdbfaeg ecadgf bcfde agcdfb cedbfg gedb cgdbf def acfeb | abfcdge ed bdge ed",
"gfdac afg ecgfba af ecadbg bfda adcgb dbcfaeg bdcagf cgfed | gabcef bfad gbecda degbca",
"faedg ba cbdgf ceadgf gdbfea adbgf aecdgb efab agcdebf dba | fedagc bgdfc ab afbdg",
"eadfb ecbdg ebgadc bfc gcef egbcfd fbcdega bcefd gcbdaf cf | bagdecf bgceda beadcg fecg",
"begdac eabdc cabef de facdbg acegdfb adcgb edcg dbegfa ead | gabfdc gbcfda daegfb ed",
"dfgcb ef fce gcdef bcgafe gdaefcb dcgebf dfbe agcde gdafbc | ceagbf fbgdc cfged gecfd",
"gefcd fcdeba bf fbgedac egadbc aedcb fcb bdecf fbeagc afbd | fbc fgabdec ebcdf gcdef",
"cagdeb bed gcbfe fdae cgdfbea dgfeb eafdgb ed agfbd cgfbda | dgbef gbfdeac fgbdace ed",
"bdgea eabcdf efbgc edfgcba fed df ecgdbf bedgf egacbf fdgc | efd fdbaec befcg ebcgfa",
"ced edgaf gacebd ce fdabc fgcdab cdaef cefgbda fcaebd ebfc | ec fdbcae fedag aecfd",
"fcgdb cfbdge agdefb af abgcfed decga cdbfag fcab dfgac dfa | agefdb adgcfb adceg cafb",
"cfag fgbedac gedfc cfead cgdafe gef fdebac ebdgc fg agefbd | fg egf egdcb gcaf",
"baecdg gec gefdab gcfead agcb gc acdgfbe edgbc dbcef gbade | cagb acefgd gc cabg",
"cgefb bgdface ecgbfd gcebaf bface egbfad ae fea gcea bcdaf | dbcfa fea aedbcfg ae",
"dfgabc cdfgeb ce gefcba fcbde degc cfbdg bce feadb defgbac | fdcbe bdfgce cfbdg abefdgc",
"dageb dcbfga dg eadcgfb edcg bgcead eafbd gad fceabg egcba | bgaecf acdgbe dg dag",
"edfacbg bgcadf adebgf gbdca fb becagd bfgdc cefgd dbf bafc | fb agebfd dfacgeb cbgafd",
"bedgafc cbd cefdgb ebca dbeafg dfbea bdafc cb dgfac dbaecf | bfadc adfeb fcabegd fcbda",
"gb ebgfdac fbag fbaecd fbdage ecfdg dceabg bdg gdbef afdbe | ecfabd bg gdb bgaf",
"ebdfcag dbfgae abcf bdegfc bgfdc abd edgca bdacg ba dabcfg | bagcefd gafcdb caefbgd dcefgab",
"cgfd fcbae geafd egc cg bgdcea fedbag acedfg fgeca dabegcf | caegf fcgd faegbd ecdgba",
"dacefgb gdcab cbeg ge dabecg afcde daecg dfeabg fbdcga gae | egdabc deabcg ebfcdga aeg",
"bgaced ebdg cgd abegfc gdfbcae bgace dg cfeda cgdea adfcgb | cfbage bgde dg aecgb",
"gfedabc bc gebad facgd ebcg edcgba dfabeg cdb dbagc abdcef | gaedb abdfce gbec gecb",
"ecfgd adgbcfe adgcbe bdfegc gefa dbacf cfgda ga acg fadceg | ag ga adbfc faeg",
"ced edbgf afebgcd dfegbc bgfead cfge ce gbcde abedfc cbagd | cdbag ecgf edc dce",
"aefcgb fbdcage bdfe cefdgb bcgde ef fgcad efdgc efc edacgb | dfacg cgabfe ecf dfcga",
"aed cgbaedf eacdg dfgac dfaebc gbde bcaegd aecgb ed fagbce | ed feadbc dbeafc de",
"decbaf bdeafg acbf gceda bcdgfea af afdce fgbecd fad cfbde | af bagfed eadbfc fdcae",
"ebacgd fagdb dgcefa ebg fedgcba adbeg eb abce fgdbec cdage | gcdeba ecab bcae ebfdgc",
"cfdb daf gacfe dfagbc ecfbagd dcbag fd adecbg fagdc dafgbe | cdfb dfa ceafg fgabed",
"bgfedc beadc gecdb dcegafb gb edgacf begcaf fdgb gefcd geb | ecbadgf ebcafg fgedc ebg",
"bde afcegd beadgc fcaebd edgbfac aefcd dfabe bfce gafbd be | eafdbc bcef efgdac gfabd",
"bdfgae ebfad fbc bdefc afec dbagcf cgefbda acefbd fc cgbde | cebfd fgbaed gfcadb bfdagce",
"abgf dabfec ebacg gdeac ebg bface egcabfd fgcbde bg eagfcb | agecd gcaebf beg fabcde",
"adecb aedbgc geabc gcabef fabed afbcdg cd dbc dgabfec ecgd | beacfg dcge dceg cgde",
"agcdfe egabcd cfgaedb bcgea fcgdeb eac deba agcfb ae cgbde | bfcegd fdacbeg bedcfg cfedag",
"fdebagc gdefb dfabg gcabdf ba acegdf acedgb adfcg bad abcf | fgdac fgadc bdgaec abfc",
"cbfd bc fdgeb eagcd becgd bce gbfecd dgeabf befgac ecgbdaf | bfgced cedgbf cb cegfbd",
"fedbg ebdcf aefgd fcgeda bg ebagcf bge bdefag dbga afegcdb | bfecd gdab gbad debcf",
"gcfeab bd dbe ecgabdf edfba gbacde bdfg gbaefd bgaef edfac | cefad db aecdf bfcage",
"fcedba edc dc fcda gefcab ebcaf efgbdca cbeda dfbgec gebda | ced dbgfec gdebcf aebcf",
"fgce eacdbf cbdfga gbfae deacfgb eagbd gaf feabc fbceag fg | afcgbd cfegba bagef fdcbea",
"dfabge dge cbdfea cbgd gd fdceb gfdebac agcef edgfc bfcdeg | fcdbge dg cdefg fcadbe",
"gbeca fbg cdfeb dbaceg fdacbg fgae cfegb aegcfb gf dfgbeac | abegfc fecdb efag gbeafcd",
"gacfeb cd cdbaegf aebcg dgebf bdcafe adgc bdcge aebgcd bcd | fbceda edcbg edbacg befdca",
"dgaeb afegdcb adfgce cabde gb gfaed gebf gbd dbagfe fbgcad | gbd gbdefa cgfadeb becda",
"efg gdfa ecadf eagfc cgaeb febdcg gf edcfga befadc gebfcda | gdfa ebdacf abgce fg",
"dbg cbadf gefbc egfadb bgcfd efdbgac gd ebfcgd bcgefa ecdg | dbfcg dgb fcgbd dbg",
"dbfecg facdeb fbadc gf gbdfa gfb ebdag dbafcg egdfbac gcaf | dcfbag gfca gbf fgcdbe",
"cefgab fgdb dge aegbf cbgaed gd bfdgcae dfega abfdeg deafc | feacd bdgf gbdfae eafcd",
"dcae ea facbged efa fagbc cefab aefbdc afgebd edbcfg edfcb | fbgdce cbfga efgbcda eadc",
"geafbcd fecbag fba dgbfc dbface bceadg cbdfa fa bdace eadf | af dabcfe cafdb cbaed",
"efbgdc dga gcbfd agfb gfcdab dbgfeca fegdca abedc ag badgc | cbdgf febacdg cfdbg gedbfca",
"edagfb gcda ebfadgc dgcbef bgc gc gbcadf gabfc agbdf abecf | ebgdcf cdga bafdg afcbg",
"daebg cfaegdb fgabe agedcb bda bd fcdabg aedgc eadcfg cebd | cdeb ecdga db geabf",
"dgef fcbged ef bfcage dbfcg cefdb caefbgd daceb fbe gfbadc | gdef dcgbf daefcgb dbgcf",
"ga bcfeagd afbg fgead fegabd becfdg age edbgf bgdeca dface | eag ga bgfa gfdeab",
"adge fbgcead dabcf de cdbfeg fbega fcaebg bed fabde faegbd | abfed gdea fbdae ebd",
"gae ae gcefbd bcaefgd edbgc gdfab debag dfecga gbdcea abce | egdbca afgdb ae cdeagb",
"cabgef ed fbadg ecbd edcbfa cfdaeg afedb ade ecdgabf afbec | fcgbeda cebd dabfg cafebg",
"fdbcae ceadf dcgbfe cad ebdcfag ecfbd eafcg deba ad gcfabd | fecag daeb cad ebcdf",
"cf egbfc cfbgeda gfc gbdce begafd facbge afbeg fcea gcdafb | fc fc afce cf",
"abdecg deg gcbd efgbda gacdfeb ecagf badce dg dafbce dgcea | dcaeb abcfed efacg gd",
"cbgefa dcbage abecgdf cdga fgebcd ca cbdeg eac dbcae eafdb | ace dgecb befcga cea",
"gef fg dgaebf bfeac cgfea ecfbgad bgfaec acefbd cgdae bfcg | fdaegcb dceag edbgaf fadebc",
"gbafde adefbcg fbaeg ged dg abgd dfbge gfedca caegfb cdebf | dgab baegf agfced bgfae",
"eb caefdg dacfe baed dgfbce efbdac efcdbag baecf ebc gcbaf | cbe fcbade adcef ecb",
"db deacg fabge bcagde bedc bfdagc adgbe cdeafgb bda cfgead | agdec becd abd bd",
"agcdb fcba efdacg gca bgdec fagdb bafgcd beafdg ca caefbdg | dgaebcf acdfeg cedagf fdcgba",
"bacdgfe ge dgeca ecdba bdcage cebafd efabdg egbc dfgac egd | egd gbce abecd afbdecg",
"bdfa aegcfb gfa daefg fa gaedc dcgebf bacefgd abdefg gebfd | fadb fga dgeabf bafgec",
"dfecb adcefgb dcbea bagc dfbeag bea ab degcba efgacd dcaeg | bae cfdgae daceg bfced",
"fbcde fdgb deg cdfega dg cageb gbedc afdebcg bcdfeg cebdfa | bfdec dbecf abecfd cegbd",
"fagdebc eafgcd gbaefc def abcde fdcgeb afgec fadg df dafce | fdagec deafgc df adceb",
"bcfaegd begfda bdaef efbgca cba bdec fgcda dfebac bdcfa bc | bafdc fabdceg cebd eafcbd",
"acbeg fgdbca ecdbgf abg eadgc ba adgbfce bcefga cgfeb afbe | fbgdac fadcgb cdfgba gfcbe",
"fbdceg af ebadcf aedbf fbcega dfca fea fadcbeg gaedb dbcfe | abgfce dacf fa bfegacd",
"ged abegfd aegbcd gaec fecbd decbg fgcdba egbdfca dgcab eg | eg gfedba efdcb gebafd",
"gfb begdaf cbadf fcgedab fgec fbcedg gebdca fg gcbdf degbc | gcbdf fg bgdfc fgb",
"dcgaef gca cgdabf ac gcdab fabc gfbad adefbg bgcde dbeafcg | cgbed dafegb ac ac",
"fgeacb bc bfdge bcg dfgcb dacegf badcgf cadb fcbaged gafcd | fcgad cb afgcd bgdcf",
"fadecg fb fcb gefb dgfcab adefcgb ebacfg cabfe becad gcafe | fcb cgefba caedgbf gdacfe",
"edcab fgce dgfcbea aebfc abgfdc efa fe bcagfe dbgfae abfcg | baedc facgb acgfb ceafb",
"bae aefbg dfbag afed bdcgfa fegcb afegbd agcdfbe ae bdcega | fegbc febag acgdbe fgdba",
"gea dagebc efadg adcfg cfgdba edacgfb caef ea gdbef geafdc | ea gfade feac fcae",
"gadc acfdb ag bfdge fbgad fbdcage abcfed agf bcdgfa cafbge | cdag cagd dbcaf gfedb",
"gacedb abfced fdcbe gdfbce ea fcdga eacfd dbcafge afbe aed | edcfbg faced becadg cagdf",
"dbcaf bdcfe fdgcab ca agfbd acgbed abgfedc acfg gafdeb bac | dbgecaf bfcda fgac bgdcaf",
"gbaef dbagef fab fagd bedgf bdgacfe gfdebc fcdbea af acbeg | gafd cdbgaef eacdbgf fba",
"gecd beacg bec badfce gbeacd bgcad gedfcab ce dbacgf fegab | ce acbgd ecabdf afbced",
"cfdbae bdfcage ecdfb gfebca cagbd ge cfegdb egcdb gedf geb | ge gbaecf bfcdage cbgedaf",
"fag cagefbd agce bcdfge ag fabed cgdef degfac geafd gafdbc | edfcg fcdge cegdbf gfaecdb",
"gdf bacgde eabdfcg ebadg fd caegf fecdgb adfb fgade bagefd | adbfeg ceagdb baefgdc gfead",
"cadgf cfabdg bfgaedc gbade cgdeaf cgef acdefb efd gaefd fe | def beagcfd deabg gecf",
"ag aeg bfegacd badec badg gdecaf afbced gcaeb bfegc cbeadg | baced gabd becad egcfad",
"bgad eabcdg acedfb gdfecb ab eba bcgeafd bcega edgbc gefca | egcaf gbedc ba ecbgad",
"gbacf cegfab fdbcea gaec fgc fdabg cdfbgae cg egbdfc cefab | cfg fcbgdea fabdec fbceag",
"debfg bdefga db gebfa gedcf bcafde edb cfaebg gbda cdbeagf | edafgb aegbdf egdcf bdga",
"fabdegc eacdb baceg afdceb bdefc gfbead dab fgecdb cfda da | cbage da ecfdb dceafb",
"gcfdba gacdb gabdf efbcdg cafd cbdafge dc aefbgd cdb agecb | cdegbf acbdg bcdag facd",
"ebd bfacge aegfb de edfa fdgeb aefdgbc abfged gdfbc dgbeca | fbgcd ed ed de",
"cdaef acfegd bfce eabfd fgdabec egcbad be gdfba bde eafdbc | be bed eb eb",
"de bedf bcgaed fedgbca cabgef ead cadebf cbfea fcead dfacg | geabfc eda eacbdg bgecfa",
"dfaec egfcd egc cg egfcad ebcgaf dcfbae gbfde fcebagd acdg | ceg dacfgbe begfd dbcafe",
"cdbg gacfd gb ebfad agefcb efacdg cfdeagb gdfba gbdacf bga | bdfga gdcb bfgace dbcafg",
"abfdeg fadec dagcbf gecadbf egabfc abefg befda edgb bdf db | dbeg db bdge dbfecag",
"ebdafc gbdcf geabcfd caedf gdecf fage gce bgeadc ge egfdca | egc gec gfdec adbfce",
"afdc fgdaeb dba ad acebf ecagbf daegbcf edacb eadfbc bedgc | cgeafb dgceb dba edbgaf",
"ebd be cafgde gfcbed gdaef gbcad bdfaeg efab efbgdca ebgda | edcfga dafeg cefbagd gfdae",
"cdefa fdceag gabefc afdb caebd ba begcd ebdcgaf dcebfa acb | cadfbe dacbe bca fabd",
"edga fcbade fea egfab gebfc ae dfcagb dgbaf adgfbec dfbgea | bdfagc fbceagd fae gbcfaed",
"ebgaf fgadc becfdg caeb dbfage cb gbc gadfbce bgacf gbacef | abdfeg bc begcdf ceba",
"dfgac feda ecabfgd eacfg fbdgc gad aefgdc cegbaf ad egdcba | febdgca aefd fdcga cfdag",
"daecb gdcabe bgdcfae decbfa efdcg cdbge dabg bg bcg bgfcea | ebcgd aecbfd debcfa bfeagc",
"gf dbfcg acbdf fdbeacg dgebac gedf aefcgb cfg fgcdeb cgbed | dgfe efdg defg bedgc",
"afgbdce fgcbe dgbe bcfdea gbcfd dbc febcga gecbdf dgcfa db | gacebf ebdg bcd dgeb",
"fcadge geadfbc db gdcae gebcd bdg gafbde gcbfe dbgcae abdc | abegdc gfecb begafdc bd",
"fc dacebf befcd fbegdca feca dafeb fedbag dbcge fadbgc fdc | ecfdb cfd acef edagbf",
"aeg gfebac dbcfea fgde gadcb ge fecdag egcad cfead bgedfca | fdgcae edfg dafec dfcae",
"gedca fcabdg fcabeg dgfbae eb becf fcbga eacgb bae faecgbd | fgbac fdcbgea aeb fgacb",
"fbcegd badfg fdeab agdefc abecgfd dgfbc adg ga bagc gbfcad | ga abgc efacbgd agfbd",
"acfebdg gbc gc cgbfae dgbea adgbc bdafc dgbeca bfgade gdce | dabfecg dagbe ebgad egcd",
"cgfe gdbfae bfaecd adgcbfe bgdca fg deacf gcfad fcgead dfg | faebdg fdcaeb fg gfce",
"cdebgf gb gfbe dgcbf bagced fcdge bdg bdfac fdcega dbagcfe | fgedc gdcef bcdegf cdfgae",
"fgcd cfa edcfab dafcge efgab gbceda daebcfg agfce cf adecg | gafce fcgaed fac afc",
"cedgba ac cabf egadf fbegcd aebfcd ecdbf dca cgadfbe dcefa | ac bgfced fecbd cbfadge",
"dagb ebcgd ad bcegdfa dbcea edgfac dgceab cgbfed bcfea acd | bdfgce bcaef da bgedfc",
"fgdea fgcade ebcagf cg eagbfd gce ecfgd cdebagf dcga cdebf | gc gc bfadge adcg",
"fbe debg fgced fcbdae gcfab dcgfae egcbf cgabedf cdefbg eb | ebf debacf be cafbg",
"fcegba cebadf efgd gaefc adfceg ade fcegabd aedgc dgabc de | aegbfc faegdbc egfd dgfe",
"df bfcdag dacbefg fagdbe begaf ceadb adf afbde gdef ecbfga | geafb faebcg ebadf df",
"ecfdab dcfba debfa ecfd dbacegf bcf fgcbae cbdga agedbf cf | dcbag eagbdfc edbgfa cafdb",
"fed bgdfac edbafc egfab gdcfb de fgedbac gced bgedf bdcefg | defcgb gbfdec cedg ed",
"gbfcdae dcab ba bgfedc ecbafd agfedb fcebd abfce cfega baf | bfa gebcdaf bfedag ebfgdc",
"afdceg eacfg cbgfaed gc dceg fgdcab egfda gbeadf abcfe fgc | gdcaef gfc fdgaec gc",
"bfdaegc bcaefd edfgcb edfgc fdgac faeg af cfa afgdce bagcd | geadcf fdabec acf agdcb",
"ebfdc dbag eabcdfg gcd gfbaec agcfbd dagfec abfcg gd cgbfd | fgabc bgad adgb fbdgc",
"efgdcb dacbg afcbe cfd fd gadceb agfd acdfb ebagfcd dgcbaf | cebgdf gdaf bfcdeg afbdcg",
"defgb bfce dgeab ef dcbgf gcefad bfecgd dfgabc eacdbfg def | bfdgc fgcbeda ecafgd fdcbge",
"edg bfdgeac bagce gbdae egadfb ed dfbe cbgfad dgfab gfcade | ed dcgaef acbeg ged",
"abedc gafbed bgaecd beadfgc bf dfb gdcfa abfcd ecfabd ebcf | dcafg fceb dbacfe fbdgace",
"eagfc acebg abedfgc feadbg adgef dfca edgacf fcg cf dfgebc | fcdgeb fgc cdbfge gdcbafe",
"bcde dcafgbe ecf debfa ec bgfca bcfae bcdfae gbfdea caegdf | cef edfab cfbdae cfgab",
"fdecag cd cbfage aedbcg gbcaedf cabdg dgc bagec bgfad dbce | fcbgae gbafd cgd bcfgae",
"feba agfdceb gbfedc gbfca ab bca cadgf gbcfe debagc cgbaef | cadgf cfaegb gbacf dcabeg",
"adceb cdgbaef cefdab ec edc dafbeg ecfbgd gdacb feca aedfb | bdfegac eafc badfe ec",
"bacdfg fcbeag afdc fabegcd acgbf agbdf dag ebdfg ad edbgca | gedbf bgdcaf fdac dcfa",
"aefg dgecb eafgdc gefdc afcdbe befcdga cdfag cef gbcadf fe | ebadfc efcbda efgcda bfgadce",
"decf fbgdca ecbgfd gbdae df bcfeg dgbef dbf becgfad acgebf | fdb fbgec dbgfac fcgdbea",
"gcaefd decab cbfea egfdbca dafb efgcb bcdega ebacfd afc fa | ecbfad caf dbeca caf",
"abfdgc gebaf cb bca fdabceg becf ecadg bacge afedgb agcefb | daceg dgfeba bc cgdea",
"cefgdb edfagc gc fdgbc aebcdf ebcdf fgc gbadf deagcfb gecb | gfc efacdb cg bacefd",
"gbead cgdeb cg fbegcad bcefd cbfdeg ecfbga gec dgcf fecbad | cg geadb gce gc",
"agebf cafeg dbfagc gecd cg gfceadb cga gafcde defac fbdcea | edcaf efdac eagcf cagfbd",
"gbdefa eb cbfdag cdeb bcaefdg dcbfae febac feb gcafe adfbc | eafgc beacf gadfeb ebf",
"abcfe ecgbfad ebda eca gfacb aecfgd dgfceb ea fbcde fdabec | acdbef deab ebcdf dbae",
"cdaf abgfed ebcdgaf deabc fbcea cefabd edgbac fa afe fgbce | acbfe bcgeda acbef abdcge",
"eabcf cd gdbecf gfcaedb dcfg gafdeb fdbge cadebg edbfc dcb | bdcegf badcegf dbc gfecbd",
"dagfb bfecga efdcb ebfcagd cg cbg ebcdaf cdgbf dcbgef egcd | agfdbec gc ebfcad edcg",
"cfaeg fd bgead efcbgd gdefa fcda cgeadbf gfd gfbeac adegfc | facd dfgae fd degba",
"egfbd cagde fbeagcd fa aedfg ceagfd daf cfag cfedba abgdce | afcg af af fdega",
"gcdfae aebdfg gfc abgec dcbf abfcg fc bdfceag fbdag bacgfd | afbged cf cfabg gfc",
"fcbaged bafce cfag gbcdfe abecg cfb gbafce dgcabe dbefa cf | cf cgfa bcf fc",
"cfed bdgfc bcd fcbegd eagcfb bdfag cd efcgb bdcgae dfcaegb | cbfge dcb gcdabe gcdbf",
"cdg afbcg adec adgefb gebdfc dc gafde adgcf fgdeac gdfbcea | caed fadge ecbdfg ceda",
"ba gfbca fdcgba egdcabf agbdce fgeacd agb cfadg fecgb fdab | gcdbfa fadb agb ba",
"cbfda ga cfbag adbcge acg aefg abfgce gefbc gdbecf cadebgf | ag dbfac agc bcgeaf",
"bgcdea fgdce cg cfga defcb gadfeb gacfedb gcd edagf gafdce | fagecdb gedfab gbecda facg",
"fcbeg gcabe bcfed gf gfbedc cfabed bfcgead gefd fgb fgcbad | acbeg adebfc bgf bgf",
"bgcfed gedbf gf egf bgeacd decbg bcfgea cdgf bfcgade bafed | bgfdce dbaef gebcd bfcgae",
"abecd bfcda gcdeba dfbe efgbac fcagd bcf fgebdac fdbace fb | fgcbea cfb bfc decab",
"fc cadebf acgfb dfgabc bfgae gcdf fbceagd adcgb cbf adecgb | fdbagec fc aedcgb cf",
"fe abgdef daefg bgcadfe dagbf dfbegc bfae gdbafc acedg gef | dagcfb baef cgedfb cabfgd",
"gef fegacd cbefd daecbg agfd cdefg fg gecbafd dceag cfebag | ecgad acbdeg dcfbe gdecba",
"cae fdeag gdcbae cagdfbe ce daefcg fgce acfdb egafbd acfde | gbdaec gafdce ce eac",
"fdaegc dbefgca dcfa fedag fdbegc fd dcagbe edf fgeba cedag | efdcag fed dcgbfea df",
"afcegb cadbef efb beadc fcdgb ef fbdce dfea agdcbfe gceabd | dcafbe eadf fcebga baced",
"cbdfgae cfaeg ebgdfa eg gbafc decg gea cbafed afgdce fcade | dceafg bdaefc ge faced",
"acbdfg bfc bc dbcg afbcd bfcdgea abdgf cfeagb fcead dfabeg | egcbaf egafdb agfbd egafcb",
"aefgdc bcfde gacb cgf bacefgd baegf ebfdga agcbfe cg ecgfb | fecadg bgfae aegdcf egfcba",
"fdgcaeb facdbg dfc dgcab cf caefbd cebdga edgfa afcdg fbgc | dfc cf dfcga bfgc",
"cdgebf fbegc cgfdea ecagfb gfbac aedbfcg ac fbagd eabc cga | faegcb ca abgcf ebfcg",
"acbgfd fecgdb bcagf abeg ecdfa gcfae ge ecg cgbadfe fgbace | gec ge cgfba dfcae",
"aef efadgc cagedb cfde dagef ef aedcg ecbagf dcfabeg afdbg | ceagd afe fae gefad",
"dbacfe cbf acefg abecdg fdbgcae bdfe bfeca bf fgdacb bdcae | cedab cefadb edfb fcbgaed",
"ebgf acdbfe adfgc baf fb fgeabc adfbecg abecg bgafc cedabg | fb bf gceadb dafgc",
"fbcead gadb adebgfc ba aegcb caegf cab ecgfdb cbdeg aegbdc | decabg edcgb cgbfde dcgeb",
"bafegdc becgdf cdebfa ecg bfedc gc fcbge eafgdc cdbg aefgb | fbecd acfedb gbcd cfagde",
    ]
}
