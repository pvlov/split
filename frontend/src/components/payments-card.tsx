import { useState } from "react";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./ui/table";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "./ui/pagination";

type Payment = {
  id: number;
  from: string;
  to: string;
  amount: number;
  context: string;
};

type PaymentsCardProps = {
  payments: Payment[];
};

const PAGE_SIZE = 5;

export function PaymentsCard({ payments }: PaymentsCardProps) {
  const [index, setIndex] = useState(0);

  const nextPage = () => setIndex((prev) => prev + PAGE_SIZE);
  const previousPage = () => setIndex((prev) => prev - PAGE_SIZE);

  const paymentsToDisplay = payments.slice(index, index + PAGE_SIZE);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Latest Payments</CardTitle>
      </CardHeader>

      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[100px]">From</TableHead>
              <TableHead>To</TableHead>
              <TableHead>Context</TableHead>
              <TableHead className="text-right">Amount</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {paymentsToDisplay.map((payment) => (
              <TableRow key={payment.id}>
                <TableCell className="font-medium">{payment.from}</TableCell>
                <TableCell>{payment.to}</TableCell>
                <TableCell>{payment.id}</TableCell>
                <TableCell className="text-right">{payment.amount}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
      <CardFooter>
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious href="#" onClick={previousPage} />
            </PaginationItem>
            <PaginationItem>
              <PaginationLink isActive={index % PAGE_SIZE == 1} href="#">1</PaginationLink>
            </PaginationItem>
            <PaginationItem>
              <PaginationLink isActive={index % PAGE_SIZE == 2} href="#">2</PaginationLink>
            </PaginationItem>
            <PaginationItem>
              <PaginationLink isActive={index % PAGE_SIZE == 3} href="#">3</PaginationLink>
            </PaginationItem>
            <PaginationItem>
              <PaginationEllipsis />
            </PaginationItem>
            <PaginationItem>
              <PaginationNext href="#" onClick={nextPage} />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      </CardFooter>
    </Card>
  );
}
