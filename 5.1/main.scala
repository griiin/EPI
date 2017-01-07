object Main {
  def checkParity(nb: Int, isEven: Boolean = true): Boolean = nb match {
    case 0 => isEven
    case _ => {
      val newIsEven = if ((nb & 1) == 1) {
        !isEven
      } else {
        isEven
      }
      checkParity((nb >> 1), newIsEven)
    }
  }

  def printParity(nb: String): Unit = {
    println(s"$nb is ${checkParity(Integer.parseInt(nb, 2))}")
  }

  def main(args: Array[String]): Unit = {
    printParity("10")
    printParity("1010")
    printParity("10110")
  }
}
