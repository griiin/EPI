object Main {
  def getBitWeigth(nb: Int, weight: Int = 0): Int = nb match {
    case 0 => weight
    case n => getBitWeigth(nb & (nb - 1), weight + 1)
  }

  def getClosestIntByWeight(nb: Int, d: Int = 1): Int = (nb, d) match {
    case (0, _) => 1
    case _ => {
      val left = nb - d;
      val right = nb + d;
      if (left > 0  && getBitWeigth(nb) == getBitWeigth(left)) {
        left
      } else if (getBitWeigth(nb) == getBitWeigth(right)) {
        right
      } else {
        getClosestIntByWeight(nb, d + 1)
      }
    }
  }

  def getBitWeigth(nb: String): Int = {
    getBitWeigth(Integer.parseInt(nb, 2))
  }

  def main(args: Array[String]): Unit = {
    /*println(getBitWeigth("1001"));
    println(getBitWeigth("1001001"));
    println(getBitWeigth("1111"));
    println(getBitWeigth("0"));*/
    println(getClosestIntByWeight(6))
    println(getClosestIntByWeight(7))
  }
}
