import { AppSidebar } from "./components/app-sidebar";
import { PaymentsCard } from "./components/payments-card";
import { ThemeProvider } from "./components/theme-provider";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
} from "./components/ui/breadcrumb";
import { Card } from "./components/ui/card";
import { Separator } from "./components/ui/separator";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "./components/ui/sidebar";

const mockPayments = [
  {
    id: 1,
    from: "Alice",
    to: "Bob",
    amount: 100,
    context: "Rent",
  },
  {
    id: 2,
    from: "Bob",
    to: "Alice",
    amount: 50,
    context: "Food",
  },
  {
    id: 3,
    from: "Alice",
    to: "Bob",
    amount: 200000,
    context: "abababababababababa",
  },
  {
    id: 4,
    from: "Alice",
    to: "Bob",
    amount: 200000,
    context: "abababababababababa",
  },
  {
    id: 5,
    from: "Alice",
    to: "Bob",
    amount: 200000,
    context: "abababababababababa",
  },
  {
    id: 6,
    from: "Alice",
    to: "Bob",
    amount: 200000,
    context: "abababababababababa",
  },
  {
    id: 7,
    from: "Alice",
    to: "Bob",
    amount: 200000,
    context: "abababababababababa",
  },
  {
    id: 8,
    from: "Alice",
    to: "Bob",
    amount: 200000,
    context: "abababababababababa",
  },
];

const AppContent: React.FC = () => {
  return (
    <>
      <AppSidebar />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
          <div className="flex items-center gap-2 px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <Breadcrumb>
              <BreadcrumbList>
                <BreadcrumbItem>Group name</BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>
          </div>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
          {/* Top half: Three equal cards */}
          <div className="flex-1 grid grid-cols-2 gap-4">
            <PaymentsCard payments={mockPayments} />
            <Card />
          </div>
          {/* Bottom half: One full-width card */}
          <div className="flex-1">
            <Card className="w-full h-full" />
          </div>
        </div>
      </SidebarInset>
    </>
  );
};

function App() {
  if (!navigator.onLine) {
    return <h1>You seem to be offline :(</h1>;
  }

  return (
    <ThemeProvider>
      <SidebarProvider>
        <AppContent />
      </SidebarProvider>
    </ThemeProvider>
  );
}

export default App;
