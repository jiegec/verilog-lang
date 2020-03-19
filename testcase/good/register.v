module register (
    input wire i,
    input wire clk,
    output reg o
);

    always @ (posedge clk) begin
        o <= i;
    end

endmodule