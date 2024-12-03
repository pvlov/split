import { RegisterForm } from "@/components/authentication";
import { ThemeProvider } from "@/components/theme-provider";
import {} from "@/components/theme-provider";
import "@/App.css";

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <RegisterForm />
    </ThemeProvider>
  );
}

export default App;
