object Main {
  def getBitWeigth(nb: Int, weight: Int = 0): Int = nb match {
    case 0 => weight
    case n => getBitWeigth(nb & (nb - 1), weight + 1)
  }

  def getBitWeigth(nb: String): Int = {
    getBitWeigth(Integer.parseInt(nb, 2))
  }

  def main(args: Array[String]): Unit = {
    println(getBitWeigth("1001"));
    println(getBitWeigth("1001001"));
    println(getBitWeigth("1111"));
    println(getBitWeigth("0"));
  }
}
