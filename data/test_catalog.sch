BEGIN
customer
customer.tbl
c_custkey Int
c_name String
c_address String
c_nationkey Int
c_phone String
c_acctbal Double
c_mktsegment String
c_comment String
END

BEGIN
orders
orders.tbl
o_orderkey Int
o_custkey Int
o_orderstatus String
o_totalprice Double
o_orderdate String
o_orderpriority String
o_clerk String
o_shippriority Int
o_comment String
END

BEGIN
lineitem
lineitem.tbl
l_orderkey Int
l_partkey Int
l_suppkey Int
l_linenumber Int
l_quantity Double
l_extendedprice Double
l_discount Double
l_tax Double
l_returnflag String
l_linestatus String
l_shipdate String
l_commitdate String
l_receiptdate String
l_shipinstruct String
l_shipmode String
l_comment String
END
