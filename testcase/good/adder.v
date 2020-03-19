module adder (
    a,
    b,
    o,
    c
);

    input wire a;
    input wire b;
    output wire o;
    output wire c;

    assign {o, c} = a + b;

endmodule