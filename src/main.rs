
/*
from math import pi, sin, ceil
from sys import stdout

raw = open("bpsk.raw", 'wb')
sample_rate = 44100

with open("INIT.CMD", 'r') as file:
    mesg = ''.join(file.readlines())
    
# Send 8 bits per character
symbols = [int(bit) for byte in mesg for bit in bin(ord(byte))[2:].zfill(8)]


"""
mesg = "Welcome to Wikipedia, the free encyclopedia that anyone can edit."
psk31_varicode = {
    0: "1010101011",
    1: "1011011011",
    2: "1011101101",
    3: "1101110111",
    4: "1011101011",
    5: "1101011111",
    6: "1011101111",
    7: "1011111101",
    8: "1011111111",
    9: "11101111",
    10: "11101",
    11: "1101101111",
    12: "1011011101",
    13: "11111",
    14: "1101110101",
    15: "1110101011",
    16: "1011110111",
    17: "1011110101",
    18: "1110101101",
    19: "1110101111",
    20: "1101011011",
    21: "1101101011",
    22: "1101101101",
    23: "1101010111",
    24: "1101111011",
    25: "1101111101",
    26: "1110110111",
    27: "1101010101",
    28: "1101011101",
    29: "1110111011",
    30: "1011111011",
    31: "1101111111",
    32: "1",
    33: "111111111",
    34: "101011111",
    35: "111110101",
    36: "111011011",
    37: "1011010101",
    38: "1010111011",
    39: "101111111",
    40: "11111011",
    41: "11110111",
    42: "101101111",
    43: "111011111",
    44: "1110101",
    45: "110101",
    46: "1010111",
    47: "110101111",
    48: "10110111",
    49: "10111101",
    50: "11101101",
    51: "11111111",
    52: "101110111",
    53: "101011011",
    54: "101101011",
    55: "110101101",
    56: "110101011",
    57: "110110111",
    58: "11110101",
    59: "110111101",
    60: "111101101",
    61: "1010101",
    62: "111010111",
    63: "1010101111",
    64: "1010111101",
    65: "1111101",
    66: "11101011",
    67: "10101101",
    68: "10110101",
    69: "1110111",
    70: "11011011",
    71: "11111101",
    72: "101010101",
    73: "1111111",
    74: "111111101",
    75: "101111101",
    76: "11010111",
    77: "10111011",
    78: "11011101",
    79: "10101011",
    80: "11010101",
    81: "111011101",
    82: "10101111",
    83: "1101111",
    84: "1101101",
    85: "101010111",
    86: "110110101",
    87: "101011101",
    88: "101110101",
    89: "101111011",
    90: "1010101101",
    91: "111110111",
    92: "111101111",
    93: "111111011",
    94: "1010111111",
    95: "101101101",
    96: "1011011111",
    97: "1011",
    98: "1011111",
    99: "101111",
    100: "101101",
    101: "11",
    102: "111101",
    103: "1011011",
    104: "101011",
    105: "1101",
    106: "111101011",
    107: "10111111",
    108: "11011",
    109: "111011",
    110: "1111",
    111: "111",
    112: "111111",
    113: "110111111",
    114: "10101",
    115: "10111",
    116: "101",
    117: "110111",
    118: "1111011",
    119: "1101011",
    120: "11011111",
    121: "1011101",
    122: "111010101",
    123: "1010110111",
    124: "110111011",
    125: "1010110101",
    126: "1011010111",
    127: "1110110101"
}

try:
    symbols = [int(bit) for char in mesg for bit in psk31_varicode[ord(char)]]
except KeyError:
    print(f"Message contains character not in PSK31 varicode! '{char}'")
    exit(1)
print(symbols)
"""

symbol_length = 2 #31.25 # PSK31
frequency = 1000 # Signal frequency in Hertz
Lof = 1 # I/Q local oscillator frequency (idk what to set here)

num_samples = ceil(sample_rate*(1/frequency)*len(symbols)*symbol_length)
for i in range(0, num_samples):
    x = (i/sample_rate)*frequency * 2*pi
    
    # Read symbol for this sample
    symbol = symbols[int(i/(sample_rate/frequency)/symbol_length)]
    
    # Shift phase of sine wave by either 0 or 180 degrees
    # TODO add cosine filtering to remove key clicks
    sample = int(sin(x - symbol*pi)*127+127)
    
    """
    # TODO I/Q demodulation
    # local oscillator
    Lo = sin(x * Lof)
    # 90 degree phase shift
    Lop = sin(x*Lof - 90/180*pi)
    """
    
    # Output as unsigned PCM mono
    #raw.write(sample.to_bytes(length=1, byteorder='big'))
    stdout.buffer.write(bytes([sample])) #.to_bytes(length=1, byteorder='big'))
*/

// std::f64::consts::PI
// std::f64 sin ceil
fn main() {
    println!("Hello, world!");

    let sample_rate: f64 = 44100.0; // Target sample rate
    let symbol_length: f64 = 2.0; //31.25; // PSK31
    let freq: f64 = 1000.0; // Signal frequency in Hertz
    let local_osc_freq: f64 = 1.0; // I/Q local oscillator frequency

    let mut mesg: Vec<u8> = Vec::new();
    //mesg.extend(b"Hello World!"); 
    mesg.extend(b"A");
    println!("{:?}", &mesg);

    let mut symbols: Vec<u8> = Vec::new();
    // For each letter in the message...
    for char in mesg {
        // Extract all the bits
        for bit_pos in 0..8 {
            let bit = char >> (7-bit_pos) & 1; 
            //print!("{:?}", bit);
            symbols.push(bit);
        }
        //println!(": {:?}", char);
    }
    println!("{:?}", &symbols);

    //let num_samples: u64 = (sample_rate * (1.0/freq) * symbols.len() as f64 * symbol_length).ceil() as u64;
    // *
    let mut num_samples: f64;
    num_samples = 1.0/freq;
    println!("1/f {:?}", num_samples);
    num_samples *= sample_rate;
    println!("sample_rate {:?}", num_samples);
    num_samples *= symbols.len() as f64;
    println!("symbols.len() {:?}", num_samples);
    num_samples *= symbol_length;
    println!("symbol_length {:?}", num_samples);
    num_samples = num_samples.ceil();
    println!("ceil() {:?}", num_samples);
    println!("bytes: {:?}", num_samples/8.0);
    //*/
    //std::io::stdout()
}
