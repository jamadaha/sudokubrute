mod solve;

use rstest::rstest;
use sudokubrute::board::Board;

#[rstest]
#[case("004300209005009001070060043006002087190007400050083000600000105003508690042910300")]
#[case("040100050107003960520008000000000017000906800803050620090060543600080700250097100")]
#[case("600120384008459072000006005000264030070080006940003000310000050089700000502000190")]
#[case("497200000100400005000016098620300040300900000001072600002005870000600004530097061")]
#[case("005910308009403060027500100030000201000820007006007004000080000640150700890000420")]
#[case("100005007380900000600000480820001075040760020069002001005039004000020100000046352")]
#[case("009065430007000800600108020003090002501403960804000100030509007056080000070240090")]
#[case("000000657702400100350006000500020009210300500047109008008760090900502030030018206")]
#[case("503070190000006750047190600400038000950200300000010072000804001300001860086720005")]
#[case("060720908084003001700100065900008000071060000002010034000200706030049800215000090")]
#[case("004083002051004300000096710120800006040000500830607900060309040007000205090050803")]
#[case("000060280709001000860320074900040510007190340003006002002970000300800905500000021")]
#[case("864371259325849761971265843436192587198657432257483916689734125713528694542916378")]
#[case("346179258187523964529648371965832417472916835813754629798261543631485792254397186")]
#[case("695127384138459672724836915851264739273981546946573821317692458489715263562348197")]
#[case("497258316186439725253716498629381547375964182841572639962145873718623954534897261")]
#[case("465912378189473562327568149738645291954821637216397854573284916642159783891736425")]
#[case("194685237382974516657213489823491675541768923769352841215839764436527198978146352")]
#[case("289765431317924856645138729763891542521473968894652173432519687956387214178246395")]
#[case("894231657762495183351876942583624719219387564647159328128763495976542831435918276")]
#[case("563472198219386754847195623472638519951247386638519472795864231324951867186723945")]
#[case("163725948584693271729184365946358127371462589852917634498231756637549812215876493")]
#[case("974183652651274389283596714129835476746912538835647921568329147317468295492751863")]
#[case("431567289729481653865329174986243517257198346143756892612975438374812965598634721")]
#[case("254367891893215674716984253532698147178432569649571328421753986365849712987126435")]
#[case("958274163123698754746153928674315289532789416819462375285941637397526841461837592")]
#[case("865379412924581376713642895397164528482795631156823947541236789679418253238957164")]
#[case("865714329917362845234598761142657983783941256596283174358176492429835617671429538")]
#[case("268495317194673852735128964872549631651387249943216785326951478589764123417832596")]
#[case("947812563583764192261935478156349287879621354324587619698253741712496835435178926")]
#[case("652483917978162435314975628825736149791824563436519872269348751547291386183657294")]
#[case("712984365346751829589263471624179538853642917197538642978316254461825793235497186")]
#[case("967254318184379562253186947691748235835621794742593681376415829428967153519832476")]
#[case("251479638948316752637258194365124879712983465894567213423691587179845326586732941")]
#[case("256734198891265374347198652514683729728519436963427581135942867689371245472856913")]
#[case("964532178187694235235817964629451783573986412841273596416728359352169847798345621")]
#[case("142569873385217946967438251536871429219654387478923165893746512621385794754192638")]
#[case("598326147314957628672481935753648291421539876869172453285764319936215784147893562")]
#[case("157468293238971645469532781926153478741896532583724916674385129892617354315249867")]
#[case("927384165684915237531672489769231548453768921218459673175826394392147856846593712")]
#[case("398256417476918253512743698185674329924835761763129845257361984831492576649587132")]
pub fn valid(#[case] input: &str) {
    let board = Board::from(input);
    assert!(board.is_valid())
}

#[rstest]
#[case("110000000000000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("100000000100000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("100000000010000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("220000000000000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("200000000200000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("200000000020000000000000000000000000000000000000000000000000000000000000000000000")]
pub fn invalid(#[case] input: &str) {
    let board = Board::from(input);
    assert!(!board.is_valid())
}

#[rstest]
#[case("864371259325849761971265843436192587198657432257483916689734125713528694542916378")]
#[case("346179258187523964529648371965832417472916835813754629798261543631485792254397186")]
#[case("695127384138459672724836915851264739273981546946573821317692458489715263562348197")]
#[case("497258316186439725253716498629381547375964182841572639962145873718623954534897261")]
#[case("465912378189473562327568149738645291954821637216397854573284916642159783891736425")]
#[case("194685237382974516657213489823491675541768923769352841215839764436527198978146352")]
#[case("289765431317924856645138729763891542521473968894652173432519687956387214178246395")]
#[case("894231657762495183351876942583624719219387564647159328128763495976542831435918276")]
#[case("563472198219386754847195623472638519951247386638519472795864231324951867186723945")]
#[case("163725948584693271729184365946358127371462589852917634498231756637549812215876493")]
#[case("974183652651274389283596714129835476746912538835647921568329147317468295492751863")]
#[case("431567289729481653865329174986243517257198346143756892612975438374812965598634721")]
#[case("254367891893215674716984253532698147178432569649571328421753986365849712987126435")]
#[case("958274163123698754746153928674315289532789416819462375285941637397526841461837592")]
#[case("865379412924581376713642895397164528482795631156823947541236789679418253238957164")]
#[case("865714329917362845234598761142657983783941256596283174358176492429835617671429538")]
#[case("268495317194673852735128964872549631651387249943216785326951478589764123417832596")]
#[case("947812563583764192261935478156349287879621354324587619698253741712496835435178926")]
#[case("652483917978162435314975628825736149791824563436519872269348751547291386183657294")]
#[case("712984365346751829589263471624179538853642917197538642978316254461825793235497186")]
#[case("967254318184379562253186947691748235835621794742593681376415829428967153519832476")]
#[case("251479638948316752637258194365124879712983465894567213423691587179845326586732941")]
#[case("256734198891265374347198652514683729728519436963427581135942867689371245472856913")]
#[case("964532178187694235235817964629451783573986412841273596416728359352169847798345621")]
#[case("142569873385217946967438251536871429219654387478923165893746512621385794754192638")]
#[case("598326147314957628672481935753648291421539876869172453285764319936215784147893562")]
#[case("157468293238971645469532781926153478741896532583724916674385129892617354315249867")]
#[case("927384165684915237531672489769231548453768921218459673175826394392147856846593712")]
#[case("398256417476918253512743698185674329924835761763129845257361984831492576649587132")]
fn solved(#[case] input: &str) {
    let board = Board::from(input);
    assert!(board.is_valid() && board.is_filled());
}

#[rstest]
#[case("004300209005009001070060043006002087190007400050083000600000105003508690042910300")]
#[case("040100050107003960520008000000000017000906800803050620090060543600080700250097100")]
#[case("600120384008459072000006005000264030070080006940003000310000050089700000502000190")]
#[case("497200000100400005000016098620300040300900000001072600002005870000600004530097061")]
#[case("005910308009403060027500100030000201000820007006007004000080000640150700890000420")]
#[case("100005007380900000600000480820001075040760020069002001005039004000020100000046352")]
#[case("009065430007000800600108020003090002501403960804000100030509007056080000070240090")]
#[case("000000657702400100350006000500020009210300500047109008008760090900502030030018206")]
#[case("503070190000006750047190600400038000950200300000010072000804001300001860086720005")]
#[case("060720908084003001700100065900008000071060000002010034000200706030049800215000090")]
#[case("004083002051004300000096710120800006040000500830607900060309040007000205090050803")]
#[case("000060280709001000860320074900040510007190340003006002002970000300800905500000021")]
#[case("110000000000000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("100000000100000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("100000000010000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("220000000000000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("200000000200000000000000000000000000000000000000000000000000000000000000000000000")]
#[case("200000000020000000000000000000000000000000000000000000000000000000000000000000000")]
fn not_solved(#[case] input: &str) {
    let board = Board::from(input);
    assert!(!board.is_valid() || !board.is_filled());
}
