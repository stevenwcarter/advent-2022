use advent::radio_stream::RadioStream;

fn main() {
    let puzzle_data = "hqhnqhqshhslswsffchfcfbblvlblqlggfwwqfwwqdddbbbhzhjjrqjqbjqjwqjwqwhwrrmcrrjqjjlllcvcrrnpptzpzmmswmmzrzjrrfcrfccpbpzzvrrdndllttwftwwgzzwhwggdvdnnlrnncscbcfcctchcdchdccztzgzjjtdtcdtcchrrgpgzgsgpsggvtvppdccfhcfclfcfdfnddlbbptbtdbtdtjdjgdjjzljlssgbgcgqcqzccfnftfjjsgstggsngsscvssjzjnjtntvnvssrqqhtqtvqtvvjbvvbnvvqvmvrmmdsdvdwwnbnvbnvvcgvcczcfzftfsfrfsfttldtdrrgrgmgbbdvvbqbhqbqlqsqbsbpbgbvgbgwwrswrwrwttvnnzdnzzwggrmrfmmvllzwlwzzlvlbvllgsllpnlnvncvcvqqrwwcmcczhchzczbccdwcwvwzwwvddlwlhljlqjqnncntnctczzmwmlmccpggljlssqswsmmpvmvwvrrcpcrprmprrtdrtdrrsjsmshmmdpdlppbnbvvmmflmmjvjhvhjvjsjbjwjvvblvbvlbbllwlslvljvlvdvdgvgcchnhpnhnvhhtfhhvssczztlzzgvvqqghqgqssdscsnsrsmrmmwgwmgmlmdmggbgzbgzgdzdzccghgfgddtntftddpdrdrmrjmmmzttqmttnwngggtgtqtqnqhnqqpnnntrntrnnshhjtjzzfqqblqqlblslflmlttcwtwzzrlrnrcrrrgtrrftfhthzhggwggvgvvfvddcnddfjddzqzrrvtrvrfrrgpgrgngsgddlmddzgzppdzzjzhjhjhqqbpbvvlrvrsvrvnrrsgsttndnbngnppmlppvgvfggvcvrvnnsnfnnfqfwwppnddrrfqqgbqqfmmlnlnngwgcgjgnjgjsgjgqjjnccdttpqttswwcgcmgmccrppmqmbbfwwvdwdfwfjwfjjblldsdrdvvgcvcwwllfpllcslsvssnvsnvvhnvnvwwcgwccslsbbnlbnbrrrtprptpdpdvdttgsgwswppcdchhqbqcbqbzbgbbstshthsttqgtqtpqqzhhwhghwwmbbdlbbtlltsllbvbnvvmqvqtvqvttlwtltmmrttwgtwwfdddwcchtchhbthbbclczzqbzzpvvwzwswggjddntnrnnwmnnbdbmbccqgqvvsnvvqbvbzvvzmvzzrfrlfrfhhdvddnmnnmhmqhhrhlrldljlbljjgqqvdvhhgmgvgddmmznndmmhssznsswvwdvdzdhhscssntnftfmmrggbmbmgmlltctbtbntnqqscqcscrcrhrlhhdchhzccvvbfvvpfffbwbsbjjmgmwgmgppjnpjphhlmhhpwwhwzhhhzbbzzcbchhpnnptpvphhsdsccffqbqllchlhlwlcljlppnccqsstzstswtwwljwjmmpttvqqspsjjclljqqlhqqtnnbrrsbrbjjllrmrjmjzmmclmclmmcvvbddhnnmhmbbwqqqhrqqtqmqbqdbdtdvvcscshccmffhqffdgdcgddljdjbdbjbdbrrtjjtnntpntpnpmnpmpgghffrnfrnnmdnmnhmmjcjffftrrttfwfvwvcwwrmwmpwmmnvntthmmgmbbcdclcppjhpjhjwwlppgsszqqwggmttfrfqflfsfmmmhchshmmjtjjgdjjnnsvsmsnwcgdfqljmnphlfdrhpggfqnhnszgpndhdqcgfhtdcgbsbtmhvnnrmqzqqcjdqndzbnrhwjvbvcldmnwltgpqbmstntnggtbqjzzqrfdsfttdfrcnsrpwrjrjqbgtjfmlwsrzdbdqvbtczgsjqhtgmctjfmdglfrsvqtgwpbqghzgzdfwzhbdhlmhdvhwjrdhhtjptvwpmjnmfcjdmdczmczvdqwvbgtvlwvwnvdlbqfshmlmvzzcmjbtpwpwgsqhfsgljzhbppcztfjdntzcvllqnzrqjwfjrlgvhmbpvbtqjrdzcsmcjzcdsmvcmhrbhgnscnfrfmscqsqpqplbrzhsrlsvvpmfdtdmtlrtvspmlljmfpshfmstjgnrrwmqlbnwbndcfdstrtqtnzpfqlcgrzmsnmhllljdgtmvftjttbwhqzcqwbwdbshgcqrptfjwbbfsjnvzztlbdchqrlbbrcnsswmhwphfwrbnvrncbrthprmltlwwlfpbqhdfqzwwcwjgqzdnvmhwpzpbtpwwvzcpfcsfqpwjljzzfwzmlfvhsccppzlzjlrvlpdtjpcptnvqjwtdbzrqwnfwmmjndflqqggczrfjlpdfjffctprnmhfdqvnzbfvhszzdmngnlmwzfdrbvlvjnmbllgrczssqcrhbmnpmqlrzgmqmhgsvcdlqnmlhlzvqzhnccbctslzlbcpdvqltqncpcrzwdchrqmwfwlcbcvfnnpjntfrznqdjsdtzqjjttddwvnfqmznhflblzvvtbwdzrrlqlmndzzzwnpbhhlvlbswfjtbnhlccscbnfgjtwbfdlwvzszwnwzhlbcdpvgqjcrpzsvvnfwqcqvrmhzhggvmzwggdpbfrdscjwhsdsbbjcnzldhzcvqtjrhsbfjlrlpvtcqhnnsvslfrdjvjhfhdcfzqchpvvhzbpqglqlrdttrdtndzhnzhtqndghtgmpsnhptprqzhbbdcrgmbvrvqmbptqgnmsccwmhrlpddvmhjntllwrzqwnsjchnblcgndjtmpswwgcstdftqcbhzgttrhnpvrhspjznhhvrlpdqbzzrvzjphcswhljdldvsrdhzltwgrcsvqwnhtqqjjrjgrplzmsnjhzrfbtqgdfgnbpvjrfzrrpdphgzrfbswdhzgbzswqtwwdtrvvswmjvwhqddfzjhgdqsfnwbmlcfwtmpldrdpwjwggbpmncvbghzjmpsqpvnmbhjfzzpsjdgmrtnndhzrphjzdbgrrnthtrfnspdngbdmwbnfjlsndqswfsvfqftqlgqjpfsfpmdbsjfrptvbpvflqgvlmmbchhghhrwmvdlrhlsdtvjjchwglcrwfsfnnbhjbtccbjsfwrbzrvgsbfnvzghhrqblqchjvdtrrbrwwwmfnczzmmrqdggsfrqbldbbfbbcthtsvpcnlbjjztwvhctbrjltqdwbzmrrfslbmjpnqwllbvjzgfsfqqtwbgwgclgdflshfhwggwdqlgbmdmdqglrtfwbddtsltmsvswhgjtqwtnrncpdzhnpfqcjnjzmtbjfzpjwgfbfggmsmfdhfbjctnhchpgfspthdbfpmvrmdbbspvwzqqwnmfwdnnblbcjszbccgflngnjjwsqshfbhjwgzrmvsgnrdbgwfhdvpmznpnvznfcqdclztcptjrrpbmpztwwbvlvtmngfhmfbfmrjbjzlrcvgllrlgthltstnwmffntqrwsrndlzqhztwpcwbdjjztgdzmgtthcvtvnjzzhvstfqhgddddgdsrbcqzqspjrncphhtbnslzphrtfqphffjrrlgwbrwqfzzqvzhnffcwvrncttgplshccglvchvlcbnlthhmzvcrfjvjqfsjjzzgqlpslfwngqnbgcwffpcqhlmlhvwssrpjbrcbftpzbbpptzrdqwpzpdjhtwbvwcqbfnwtflfgzgttglpcwdnzsmjffgfftdnpzlpszmzrrhvlzdfpgbzqlvvfslnndcmjvvpwzwcdzpcttfwbsdswmfqbbhbfbjvbvtspbbhmphzjsjcmmzpvvzhtvzlgwlqqvbnrgtszvghjfmchrwwhpmbsvfmgvqdtmvdtjppchbsgqrtgtnzczqmpgjdbmmflfpjbcdmhldwpgdtdvsbzhztjzfhcsbndfjntbldjnqwdffqspfnlplbtcdjtwdjhldtsdfrnmpfhzghnpcqlhhgblmqjvwhndqfbccvzzlbzbdprvcpwjjhrqnjptwssbjhgvpgtfzqwrzjvbdgwtnmptvdjrffcmbzmzcmrfbjv";

    let answer_a = a(puzzle_data);
    println!("Start of stream packet: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B: {}", b_result);
}

fn a(stream: &str) -> u32 {
    let stream: RadioStream = RadioStream::new(stream);

    stream.get_stream_packet_index()
}

fn b(stream: &str) -> u32 {
    let stream: RadioStream = RadioStream::new(stream);

    stream.get_message_packet_index()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn test_a_2() {
        assert_eq!(a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_a_3() {
        assert_eq!(a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test_a_4() {
        assert_eq!(a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn test_a_5() {
        assert_eq!(a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_b() {
        assert_eq!(b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn test_b_2() {
        assert_eq!(b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn test_b_3() {
        assert_eq!(b("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn test_b_4() {
        assert_eq!(b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn test_b_5() {
        assert_eq!(b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
