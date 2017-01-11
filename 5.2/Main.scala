object Main {
  def swapBit(nb: Int, from: Int, to: Int): Int = {
    def fromMask = 1 << from
    def toMask = 1 << to
    def fromValue = if ((nb & fromMask) != 0) 1 else 0
    def toValue = if ((nb & toMask) != 0) 1 else 0
    if (fromValue == toValue) {
      nb
    } else {
      ((nb ^ fromMask) ^ toMask)
    }
  }

  def swapAndPrint(nb: String, from: Int, to: Int): Unit = {
    println(swapBit(Integer.parseInt(nb, 2), from ,to).toBinaryString)
  }

  def main(args: Array[String]): Unit = {
    swapAndPrint("10000100", 2, 0)
    swapAndPrint("11010100", 4, 0)
    swapAndPrint("11000100", 6, 7)
    swapAndPrint("10100100", 0, 7)
    swapAndPrint("10100100", 0, 7)
  }
}


// 0 1 0 0
// 0 0 0 1

// 1 1 =>

// 1 1 => 1
// 0 1 => 1

// 1 0 => 0
// 0 0 => 0
