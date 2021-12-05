// use std::cmp;
use std::ops;

#[derive(Debug)]
struct Point(i32,i32);
#[derive(Debug)]
struct Line(Point,Point);

impl Line {
    pub fn points(&self) -> Vec<Point> {
        let Point(startx, starty) = self.0;
        let Point(stopx, stopy) = self.1;

        let mut slopex = 0;
        let mut slopey = 0;
        if startx > stopx {
            slopex = -1;
        }
        if startx < stopx {
            slopex = 1;
        }
        if starty > stopy {
            slopey = -1;
        }
        if starty < stopy {
            slopey = 1;
        }

        let mut points: Vec<Point> = vec![];
        points.push(Point(startx, starty));

        let mut p = Point(startx, starty);

        while p.0 != stopx || p.1 != stopy {
            p = p + Point(slopex, slopey);
            points.push(Point(p.0, p.1));
        }

        return points;
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0+rhs.0, self.1+rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        let line1 = Line(Point(280,66),Point(514,66)).points();
        let line2 = Line(Point(514,66),Point(280,66)).points();
        println!("LINES: {} | {}", line1.len(), line2.len());
        assert_eq!(line1.len(), line2.len());
        assert_eq!(line1.len(), 235);

        let line1 = Line(Point(66,280),Point(66,514)).points();
        let line2 = Line(Point(66,514),Point(66,280)).points();
        assert_eq!(line1.len(), line2.len());
        assert_eq!(line1.len(), 235);

        let line1 = Line(Point(66,66),Point(68,68)).points();
        let line2 = Line(Point(68,68),Point(66,66)).points();
        assert_eq!(line1.len(), line2.len());
        assert_eq!(line1.len(), 3);

        let start = line1.first().unwrap();
        assert_eq!(start.0, 66);
        assert_eq!(start.1, 66);
        let middle = &line1[1];
        assert_eq!(middle.0, 67);
        assert_eq!(middle.1, 67);
        let stop = line1.last().unwrap();
        assert_eq!(stop.0, 68);
        assert_eq!(stop.1, 68);
    }

    #[test]
    fn test_add() {
        let point = Point(2,3);
        let new = point+Point(5,2);
        assert_eq!(new.0, 7);
        assert_eq!(new.1, 5);
    }

    #[test]
    fn test_diagonal() {
        let points = Line(Point(941,230),Point(322,849)).points();

        assert_eq!(points.len(), 620);
        assert_eq!(points[0].0, 941);
        assert_eq!(points[0].1, 230);
        assert_eq!(points[619].0, 322);
        assert_eq!(points[619].1, 849);
    }

}

fn main() {
    let input = input();

    let mut world = [[0u32;1000];1000];
    let mut crossings = 0;
        
    for line in input {
        for point in line.points() {
            let Point(x,y) = point;
            world[x as usize][y as usize] += 1;
            if world[x as usize][y as usize] == 2 {
                crossings += 1;
            }
        }
    }

    println!("answer: {}", crossings)
}

fn input() -> Vec<Line> {
    vec![
Line(Point(941,230),Point(322,849)),
Line(Point(762,196),Point(701,257)),
Line(Point(656,197),Point(595,136)),
Line(Point(687,692),Point(57,692)),
Line(Point(37,953),Point(903,87)),
Line(Point(674,102),Point(84,102)),
Line(Point(952,323),Point(786,157)),
Line(Point(807,948),Point(430,948)),
Line(Point(280,66),Point(514,66)),
Line(Point(810,381),Point(928,263)),
Line(Point(41,278),Point(112,207)),
Line(Point(754,11),Point(754,574)),
Line(Point(499,830),Point(725,604)),
Line(Point(713,172),Point(658,172)),
Line(Point(805,54),Point(594,54)),
Line(Point(442,910),Point(40,508)),
Line(Point(160,170),Point(925,935)),
Line(Point(265,899),Point(265,313)),
Line(Point(960,976),Point(77,93)),
Line(Point(820,244),Point(877,187)),
Line(Point(883,501),Point(345,501)),
Line(Point(12,978),Point(941,49)),
Line(Point(988,46),Point(988,572)),
Line(Point(285,775),Point(285,298)),
Line(Point(718,69),Point(121,69)),
Line(Point(218,641),Point(146,641)),
Line(Point(857,277),Point(124,277)),
Line(Point(32,36),Point(657,36)),
Line(Point(964,280),Point(609,280)),
Line(Point(739,981),Point(910,981)),
Line(Point(960,794),Point(243,794)),
Line(Point(447,682),Point(751,378)),
Line(Point(813,103),Point(813,240)),
Line(Point(568,705),Point(497,705)),
Line(Point(888,47),Point(888,231)),
Line(Point(936,95),Point(336,695)),
Line(Point(305,349),Point(18,636)),
Line(Point(54,240),Point(54,222)),
Line(Point(28,704),Point(625,107)),
Line(Point(680,325),Point(680,623)),
Line(Point(209,405),Point(209,123)),
Line(Point(947,906),Point(947,721)),
Line(Point(149,810),Point(834,125)),
Line(Point(897,875),Point(146,124)),
Line(Point(928,267),Point(928,484)),
Line(Point(871,516),Point(871,136)),
Line(Point(954,725),Point(706,725)),
Line(Point(680,645),Point(958,645)),
Line(Point(680,326),Point(908,326)),
Line(Point(173,157),Point(890,874)),
Line(Point(842,802),Point(166,126)),
Line(Point(750,442),Point(270,922)),
Line(Point(567,891),Point(567,784)),
Line(Point(374,623),Point(374,174)),
Line(Point(979,725),Point(765,511)),
Line(Point(336,440),Point(82,440)),
Line(Point(214,213),Point(939,938)),
Line(Point(652,815),Point(763,815)),
Line(Point(220,48),Point(331,159)),
Line(Point(580,522),Point(141,522)),
Line(Point(286,685),Point(286,779)),
Line(Point(865,343),Point(865,257)),
Line(Point(738,898),Point(405,565)),
Line(Point(703,571),Point(420,571)),
Line(Point(792,368),Point(792,955)),
Line(Point(738,905),Point(738,79)),
Line(Point(646,95),Point(737,95)),
Line(Point(930,908),Point(72,50)),
Line(Point(310,933),Point(310,243)),
Line(Point(192,22),Point(918,748)),
Line(Point(245,803),Point(81,639)),
Line(Point(567,218),Point(901,218)),
Line(Point(148,950),Point(965,133)),
Line(Point(147,772),Point(159,772)),
Line(Point(774,84),Point(774,960)),
Line(Point(860,798),Point(372,798)),
Line(Point(856,131),Point(856,703)),
Line(Point(368,603),Point(247,603)),
Line(Point(587,533),Point(301,533)),
Line(Point(832,461),Point(832,506)),
Line(Point(164,709),Point(960,709)),
Line(Point(874,471),Point(327,471)),
Line(Point(346,237),Point(346,921)),
Line(Point(683,300),Point(910,527)),
Line(Point(353,717),Point(353,575)),
Line(Point(586,578),Point(798,366)),
Line(Point(27,813),Point(27,434)),
Line(Point(311,391),Point(418,391)),
Line(Point(369,304),Point(33,304)),
Line(Point(591,226),Point(591,558)),
Line(Point(634,545),Point(513,545)),
Line(Point(439,257),Point(207,257)),
Line(Point(42,791),Point(581,252)),
Line(Point(155,801),Point(155,294)),
Line(Point(599,603),Point(599,182)),
Line(Point(48,607),Point(337,896)),
Line(Point(199,828),Point(506,828)),
Line(Point(28,147),Point(733,852)),
Line(Point(799,563),Point(799,22)),
Line(Point(206,625),Point(455,874)),
Line(Point(185,330),Point(335,480)),
Line(Point(161,746),Point(590,746)),
Line(Point(932,13),Point(269,13)),
Line(Point(649,746),Point(649,309)),
Line(Point(463,169),Point(930,636)),
Line(Point(568,251),Point(386,251)),
Line(Point(739,692),Point(233,692)),
Line(Point(941,989),Point(84,132)),
Line(Point(513,356),Point(513,628)),
Line(Point(534,168),Point(285,168)),
Line(Point(447,563),Point(447,698)),
Line(Point(898,915),Point(791,808)),
Line(Point(339,405),Point(432,405)),
Line(Point(414,940),Point(335,940)),
Line(Point(591,741),Point(59,741)),
Line(Point(347,330),Point(347,341)),
Line(Point(186,40),Point(438,292)),
Line(Point(849,872),Point(295,318)),
Line(Point(406,620),Point(938,620)),
Line(Point(346,226),Point(864,226)),
Line(Point(609,40),Point(478,171)),
Line(Point(820,900),Point(947,900)),
Line(Point(201,63),Point(201,107)),
Line(Point(984,652),Point(47,652)),
Line(Point(193,204),Point(776,204)),
Line(Point(173,892),Point(740,892)),
Line(Point(389,675),Point(709,355)),
Line(Point(489,954),Point(546,954)),
Line(Point(18,82),Point(587,651)),
Line(Point(646,150),Point(675,150)),
Line(Point(618,805),Point(618,592)),
Line(Point(178,617),Point(178,606)),
Line(Point(179,30),Point(505,30)),
Line(Point(984,21),Point(21,984)),
Line(Point(172,167),Point(15,167)),
Line(Point(17,209),Point(192,209)),
Line(Point(814,945),Point(814,18)),
Line(Point(385,632),Point(161,632)),
Line(Point(126,41),Point(474,389)),
Line(Point(575,778),Point(737,778)),
Line(Point(74,270),Point(147,270)),
Line(Point(891,248),Point(467,672)),
Line(Point(95,426),Point(95,728)),
Line(Point(235,73),Point(235,583)),
Line(Point(730,302),Point(730,466)),
Line(Point(388,587),Point(377,598)),
Line(Point(525,155),Point(184,155)),
Line(Point(370,278),Point(966,874)),
Line(Point(950,150),Point(444,656)),
Line(Point(644,935),Point(401,935)),
Line(Point(798,515),Point(506,807)),
Line(Point(976,562),Point(253,562)),
Line(Point(674,350),Point(603,421)),
Line(Point(686,653),Point(576,653)),
Line(Point(691,278),Point(593,180)),
Line(Point(964,961),Point(76,73)),
Line(Point(735,582),Point(735,389)),
Line(Point(786,885),Point(76,885)),
Line(Point(402,732),Point(231,732)),
Line(Point(660,881),Point(660,525)),
Line(Point(683,383),Point(683,364)),
Line(Point(174,20),Point(174,75)),
Line(Point(692,819),Point(107,819)),
Line(Point(344,669),Point(577,902)),
Line(Point(562,126),Point(697,261)),
Line(Point(621,344),Point(621,707)),
Line(Point(731,892),Point(213,374)),
Line(Point(216,828),Point(663,828)),
Line(Point(990,534),Point(990,356)),
Line(Point(973,714),Point(519,714)),
Line(Point(25,981),Point(983,23)),
Line(Point(659,399),Point(535,275)),
Line(Point(967,885),Point(183,101)),
Line(Point(612,684),Point(732,684)),
Line(Point(955,485),Point(955,806)),
Line(Point(582,714),Point(582,719)),
Line(Point(342,203),Point(905,203)),
Line(Point(188,488),Point(272,488)),
Line(Point(659,65),Point(659,679)),
Line(Point(306,85),Point(605,384)),
Line(Point(975,847),Point(975,353)),
Line(Point(742,989),Point(742,652)),
Line(Point(917,524),Point(934,524)),
Line(Point(890,571),Point(662,799)),
Line(Point(901,791),Point(901,118)),
Line(Point(631,447),Point(114,447)),
Line(Point(850,28),Point(797,28)),
Line(Point(842,759),Point(91,759)),
Line(Point(659,538),Point(253,944)),
Line(Point(693,69),Point(693,452)),
Line(Point(161,515),Point(789,515)),
Line(Point(892,630),Point(892,785)),
Line(Point(78,947),Point(931,947)),
Line(Point(561,728),Point(11,178)),
Line(Point(138,842),Point(138,133)),
Line(Point(890,373),Point(628,373)),
Line(Point(509,370),Point(592,370)),
Line(Point(982,41),Point(185,838)),
Line(Point(184,210),Point(184,218)),
Line(Point(390,525),Point(390,558)),
Line(Point(387,151),Point(387,39)),
Line(Point(718,808),Point(833,808)),
Line(Point(206,234),Point(206,620)),
Line(Point(84,150),Point(84,959)),
Line(Point(336,468),Point(307,468)),
Line(Point(764,19),Point(739,44)),
Line(Point(752,607),Point(643,607)),
Line(Point(233,149),Point(112,149)),
Line(Point(368,612),Point(725,255)),
Line(Point(929,497),Point(909,477)),
Line(Point(829,274),Point(829,190)),
Line(Point(312,268),Point(312,128)),
Line(Point(519,18),Point(519,552)),
Line(Point(896,19),Point(140,19)),
Line(Point(368,727),Point(368,114)),
Line(Point(233,813),Point(750,813)),
Line(Point(477,758),Point(477,213)),
Line(Point(615,171),Point(615,530)),
Line(Point(38,461),Point(301,461)),
Line(Point(862,107),Point(154,815)),
Line(Point(271,52),Point(271,517)),
Line(Point(203,936),Point(365,936)),
Line(Point(96,700),Point(13,617)),
Line(Point(290,554),Point(389,455)),
Line(Point(377,923),Point(377,890)),
Line(Point(347,511),Point(147,511)),
Line(Point(889,412),Point(762,412)),
Line(Point(558,412),Point(424,412)),
Line(Point(45,838),Point(45,845)),
Line(Point(958,27),Point(958,454)),
Line(Point(154,244),Point(20,244)),
Line(Point(315,154),Point(315,173)),
Line(Point(135,618),Point(135,71)),
Line(Point(380,422),Point(131,671)),
Line(Point(314,500),Point(314,873)),
Line(Point(915,320),Point(915,159)),
Line(Point(213,772),Point(977,772)),
Line(Point(14,22),Point(978,986)),
Line(Point(444,759),Point(444,385)),
Line(Point(730,650),Point(730,210)),
Line(Point(532,551),Point(633,652)),
Line(Point(547,426),Point(335,426)),
Line(Point(868,191),Point(156,903)),
Line(Point(462,599),Point(611,748)),
Line(Point(729,709),Point(729,714)),
Line(Point(665,229),Point(849,413)),
Line(Point(880,947),Point(880,159)),
Line(Point(249,837),Point(249,604)),
Line(Point(575,205),Point(196,584)),
Line(Point(960,665),Point(320,25)),
Line(Point(617,853),Point(412,853)),
Line(Point(224,60),Point(224,467)),
Line(Point(226,741),Point(226,47)),
Line(Point(371,595),Point(118,342)),
Line(Point(371,708),Point(371,561)),
Line(Point(236,141),Point(955,860)),
Line(Point(55,509),Point(55,938)),
Line(Point(684,885),Point(684,670)),
Line(Point(93,509),Point(497,105)),
Line(Point(284,61),Point(812,61)),
Line(Point(438,353),Point(242,353)),
Line(Point(77,716),Point(363,430)),
Line(Point(283,769),Point(905,147)),
Line(Point(56,799),Point(551,799)),
Line(Point(804,637),Point(804,526)),
Line(Point(476,54),Point(154,54)),
Line(Point(686,400),Point(686,145)),
Line(Point(740,905),Point(417,905)),
Line(Point(21,113),Point(823,915)),
Line(Point(286,132),Point(880,726)),
Line(Point(923,378),Point(771,378)),
Line(Point(924,922),Point(36,34)),
Line(Point(801,609),Point(801,407)),
Line(Point(465,671),Point(550,756)),
Line(Point(628,235),Point(628,842)),
Line(Point(684,840),Point(716,808)),
Line(Point(841,366),Point(495,712)),
Line(Point(740,208),Point(740,174)),
Line(Point(657,370),Point(657,731)),
Line(Point(817,781),Point(466,781)),
Line(Point(308,894),Point(308,370)),
Line(Point(497,233),Point(755,233)),
Line(Point(35,145),Point(35,398)),
Line(Point(383,163),Point(578,163)),
Line(Point(620,985),Point(620,849)),
Line(Point(178,253),Point(178,724)),
Line(Point(556,51),Point(556,525)),
Line(Point(650,187),Point(706,243)),
Line(Point(161,988),Point(599,550)),
Line(Point(861,256),Point(501,616)),
Line(Point(46,555),Point(181,555)),
Line(Point(980,975),Point(980,916)),
Line(Point(345,751),Point(479,617)),
Line(Point(534,642),Point(534,202)),
Line(Point(901,240),Point(901,490)),
Line(Point(984,280),Point(337,927)),
Line(Point(578,663),Point(578,298)),
Line(Point(377,943),Point(259,943)),
Line(Point(975,38),Point(39,974)),
Line(Point(697,870),Point(387,560)),
Line(Point(147,520),Point(218,520)),
Line(Point(683,711),Point(486,711)),
Line(Point(825,26),Point(122,729)),
Line(Point(855,84),Point(751,84)),
Line(Point(558,945),Point(989,945)),
Line(Point(660,195),Point(597,195)),
Line(Point(889,696),Point(317,696)),
Line(Point(969,248),Point(240,977)),
Line(Point(598,625),Point(598,148)),
Line(Point(176,151),Point(256,151)),
Line(Point(939,70),Point(648,70)),
Line(Point(645,431),Point(411,431)),
Line(Point(502,518),Point(221,518)),
Line(Point(821,988),Point(213,988)),
Line(Point(361,850),Point(684,850)),
Line(Point(506,173),Point(506,405)),
Line(Point(323,151),Point(726,151)),
Line(Point(131,519),Point(35,519)),
Line(Point(164,445),Point(798,445)),
Line(Point(425,989),Point(425,133)),
Line(Point(18,739),Point(684,73)),
Line(Point(138,545),Point(138,155)),
Line(Point(401,104),Point(766,104)),
Line(Point(864,855),Point(203,855)),
Line(Point(636,361),Point(604,361)),
Line(Point(820,970),Point(820,882)),
Line(Point(866,859),Point(835,859)),
Line(Point(112,507),Point(112,715)),
Line(Point(529,494),Point(529,928)),
Line(Point(104,469),Point(193,469)),
Line(Point(82,841),Point(831,92)),
Line(Point(258,518),Point(258,778)),
Line(Point(34,917),Point(135,917)),
Line(Point(777,553),Point(985,345)),
Line(Point(64,952),Point(719,297)),
Line(Point(341,224),Point(902,224)),
Line(Point(87,128),Point(525,566)),
Line(Point(951,400),Point(448,903)),
Line(Point(344,963),Point(21,963)),
Line(Point(983,244),Point(983,503)),
Line(Point(938,771),Point(635,771)),
Line(Point(560,262),Point(560,974)),
Line(Point(46,386),Point(75,386)),
Line(Point(898,747),Point(898,17)),
Line(Point(239,929),Point(149,929)),
Line(Point(849,881),Point(849,251)),
Line(Point(204,204),Point(204,753)),
Line(Point(830,33),Point(830,130)),
Line(Point(304,339),Point(42,339)),
Line(Point(565,312),Point(773,312)),
Line(Point(387,523),Point(234,523)),
Line(Point(239,421),Point(543,725)),
Line(Point(197,433),Point(197,723)),
Line(Point(595,21),Point(370,21)),
Line(Point(547,171),Point(480,104)),
Line(Point(639,910),Point(639,241)),
Line(Point(908,185),Point(560,185)),
Line(Point(947,565),Point(947,411)),
Line(Point(211,670),Point(588,293)),
Line(Point(753,708),Point(753,624)),
Line(Point(36,147),Point(859,970)),
Line(Point(423,94),Point(930,94)),
Line(Point(613,680),Point(607,680)),
Line(Point(277,263),Point(836,822)),
Line(Point(186,413),Point(827,413)),
Line(Point(483,173),Point(142,173)),
Line(Point(25,771),Point(409,387)),
Line(Point(328,916),Point(613,631)),
Line(Point(267,604),Point(724,147)),
Line(Point(430,616),Point(150,896)),
Line(Point(692,463),Point(50,463)),
Line(Point(306,360),Point(306,653)),
Line(Point(736,948),Point(736,174)),
Line(Point(797,529),Point(774,529)),
Line(Point(492,486),Point(492,812)),
Line(Point(659,429),Point(102,429)),
Line(Point(582,503),Point(695,616)),
Line(Point(780,62),Point(780,164)),
Line(Point(58,318),Point(387,318)),
Line(Point(286,694),Point(286,396)),
Line(Point(248,241),Point(248,361)),
Line(Point(112,963),Point(707,963)),
Line(Point(771,722),Point(636,722)),
Line(Point(508,76),Point(389,76)),
Line(Point(435,307),Point(201,541)),
Line(Point(167,312),Point(618,763)),
Line(Point(721,407),Point(305,823)),
Line(Point(57,203),Point(516,203)),
Line(Point(83,239),Point(83,607)),
Line(Point(810,686),Point(137,13)),
Line(Point(817,268),Point(101,984)),
Line(Point(379,975),Point(379,631)),
Line(Point(597,38),Point(611,38)),
Line(Point(56,504),Point(56,900)),
Line(Point(108,587),Point(261,740)),
Line(Point(625,426),Point(476,426)),
Line(Point(248,486),Point(643,881)),
Line(Point(932,25),Point(21,936)),
Line(Point(388,613),Point(388,296)),
Line(Point(644,188),Point(644,273)),
Line(Point(871,425),Point(871,791)),
Line(Point(722,866),Point(722,39)),
Line(Point(96,579),Point(96,97)),
Line(Point(876,64),Point(297,643)),
Line(Point(581,633),Point(59,633)),
Line(Point(11,10),Point(989,988)),
Line(Point(947,55),Point(266,736)),
Line(Point(532,553),Point(735,756)),
Line(Point(898,855),Point(83,40)),
Line(Point(533,289),Point(306,62)),
Line(Point(497,736),Point(332,571)),
Line(Point(871,201),Point(345,727)),
Line(Point(550,686),Point(256,686)),
Line(Point(858,585),Point(607,836)),
Line(Point(380,171),Point(15,171)),
Line(Point(864,112),Point(864,686)),
Line(Point(791,857),Point(305,857)),
Line(Point(898,579),Point(741,579)),
Line(Point(479,713),Point(113,713)),
Line(Point(19,143),Point(779,903)),
Line(Point(347,161),Point(140,368)),
Line(Point(479,395),Point(534,340)),
Line(Point(929,37),Point(77,889)),
Line(Point(128,958),Point(884,202)),
Line(Point(921,18),Point(921,650)),
Line(Point(263,550),Point(263,280)),
Line(Point(155,592),Point(235,592)),
Line(Point(565,34),Point(565,454)),
Line(Point(913,371),Point(173,371)),
Line(Point(199,158),Point(974,933)),
Line(Point(98,775),Point(98,234)),
Line(Point(649,576),Point(649,444)),
Line(Point(801,855),Point(548,855)),
Line(Point(859,913),Point(363,913)),
Line(Point(274,487),Point(274,654)),
Line(Point(729,982),Point(443,982)),
Line(Point(664,827),Point(77,240)),
Line(Point(656,885),Point(656,350)),
Line(Point(916,74),Point(284,706)),
Line(Point(439,31),Point(439,175)),
Line(Point(423,753),Point(280,753)),
Line(Point(424,914),Point(948,914)),
Line(Point(980,723),Point(980,674)),
Line(Point(656,437),Point(626,407)),
Line(Point(577,654),Point(423,654)),
Line(Point(19,224),Point(424,224)),
Line(Point(310,181),Point(704,575)),
Line(Point(828,296),Point(828,308)),
Line(Point(905,151),Point(955,151)),
Line(Point(319,178),Point(892,178)),
Line(Point(972,939),Point(65,32)),
Line(Point(497,98),Point(91,98)),
Line(Point(987,402),Point(943,446)),
Line(Point(904,19),Point(174,749)),
Line(Point(265,885),Point(265,835)),
Line(Point(475,414),Point(658,597)),
Line(Point(610,93),Point(938,93)),
Line(Point(961,892),Point(661,892)),
Line(Point(297,600),Point(378,600)),
Line(Point(405,637),Point(52,284)),
Line(Point(439,874),Point(439,612)),
Line(Point(275,185),Point(275,218)),
Line(Point(220,840),Point(220,735)),
Line(Point(372,153),Point(644,425)),
Line(Point(896,964),Point(896,461)),
Line(Point(916,484),Point(951,449)),
Line(Point(485,355),Point(456,355)),
Line(Point(198,793),Point(198,132)),
Line(Point(614,735),Point(561,735)),
Line(Point(181,591),Point(147,591)),
Line(Point(175,289),Point(159,289)),
Line(Point(899,758),Point(962,695)),
Line(Point(506,647),Point(506,858)),
Line(Point(443,828),Point(720,828)),
Line(Point(623,641),Point(623,631)),
Line(Point(202,409),Point(891,409)),
Line(Point(486,751),Point(80,345)),
Line(Point(781,73),Point(781,710)),
Line(Point(911,643),Point(911,571)),
Line(Point(799,151),Point(89,861)),
Line(Point(716,815),Point(810,815)),
Line(Point(947,517),Point(947,575)),
Line(Point(704,260),Point(704,727)),
Line(Point(113,581),Point(113,606)),
Line(Point(408,252),Point(408,761)),
Line(Point(601,753),Point(457,609)),
Line(Point(851,424),Point(501,774)),
Line(Point(670,941),Point(916,941)),
Line(Point(480,839),Point(205,564)),
Line(Point(912,949),Point(38,75)),
Line(Point(477,39),Point(925,487)),
Line(Point(139,898),Point(309,898)),
Line(Point(93,386),Point(93,194)),
Line(Point(184,132),Point(943,891)),
Line(Point(247,557),Point(247,182)),
Line(Point(832,22),Point(76,778)),
Line(Point(61,814),Point(806,69)),
Line(Point(816,640),Point(604,428)),
Line(Point(214,561),Point(623,152)),
Line(Point(698,858),Point(389,858)),
    ]
}
