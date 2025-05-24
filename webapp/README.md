# ShExML2RML WebApp

A modern, interactive web interface for converting ShExML (Shape Expressions Mapping Language) to RML (RDF Mapping Language) built with React, TypeScript, and Vite.

## 🌟 Features

- ✨ **Interactive Editor** - Real-time ShExML editing with syntax highlighting
- 🔄 **Live Transpilation** - Instant conversion from ShExML to RML
- 📋 **Multiple Examples** - Pre-loaded basic and advanced ShExML examples
- 🎨 **Modern UI** - Beautiful, responsive design with dark/light theme support
- 📥 **Export Functionality** - Download generated RML files
- 🔍 **Error Handling** - Clear error messages and validation
- 📱 **Mobile Friendly** - Responsive design that works on all devices

## 🛠️ Tech Stack

- **Frontend**: React 19.1.0 + TypeScript
- **Build Tool**: Vite 6.3.5
- **UI Components**: Radix UI + Tailwind CSS 4.1.7
- **Icons**: Lucide React
- **Theming**: Next Themes
- **Notifications**: Sonner
- **Testing**: Vitest + React Testing Library

## 🚀 Quick Start

### Prerequisites

- Node.js 20+ installed
- npm or yarn package manager

### Development

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Start development server:**
   ```bash
   npm run dev
   ```

3. **Open in browser:**
   ```
   http://localhost:5173
   ```

### Production Build

```bash
# Build for local production
npm run build

# Build for production with remote API
npm run build:prod
```

### Preview Production Build

```bash
npm run preview
```

## 🧪 Testing

The webapp includes comprehensive tests covering components, utilities, and integrations.

```bash
# Run tests in watch mode
npm test

# Run tests once
npm run test:run

# Run tests with UI (if available)
npm run test:ui

# Run tests with coverage
npm run test:coverage
```

**Test Coverage:**
- ✅ Component rendering
- ✅ User interactions
- ✅ API integration
- ✅ Error handling
- ✅ Utility functions
- ✅ UI snapshots

## 📁 Project Structure

```
webapp/
├── public/                 # Static assets
│   └── ferris.svg         # Rust mascot icon
├── src/
│   ├── components/        # React components
│   │   ├── ui/           # Reusable UI components
│   │   ├── transpiler.tsx # Main transpiler component
│   │   ├── theme-provider.tsx
│   │   └── mode-toggle.tsx
│   ├── lib/              # Utility functions
│   ├── test/             # Test files
│   ├── App.tsx           # Main app component
│   └── main.tsx          # Entry point
├── package.json
├── vite.config.ts        # Vite configuration
├── vitest.config.ts      # Test configuration
└── README.md
```

## 🔧 Configuration

### Environment Variables

- `VITE_API_URL` - API endpoint URL (default: `http://localhost:8080`)

### API Integration

The webapp communicates with the Rust backend API:

- **Endpoint**: `POST /transpile`
- **Content-Type**: `text/plain`
- **Response**: RML output or error message

### Styling

- **Framework**: Tailwind CSS 4.1.7
- **Components**: Radix UI primitives
- **Theming**: CSS variables with dark/light mode support
- **Animations**: CSS animations with `tw-animate-css`

## 🎨 UI Components

The webapp uses a modern component library including:

- **Button** - Various styles and sizes
- **Card** - Content containers
- **Tabs** - Navigation between views
- **Badge** - Status indicators
- **Tooltip** - Contextual information
- **Theme Toggle** - Dark/light mode switcher

## 📋 Usage Examples

### Basic ShExML Example
```shexml
PREFIX : <http://example.com/>
PREFIX schema: <http://schema.org/>
SOURCE films_xml_file <http://shexml.herminiogarcia.com/files/films.xml>

ITERATOR film_xml <xpath: //film> {
    FIELD id <@id>
    FIELD name <name>
}

EXPRESSION films <films_xml_file.film_xml>

:Film :[films.id] {
    schema:name [films.name] ;
}
```

### Generated RML Output
```ttl
@prefix rml: <http://semweb.mmlab.be/ns/rml#> .
@prefix rr: <http://www.w3.org/ns/r2rml#> .
@prefix ql: <http://semweb.mmlab.be/ns/ql#> .
@prefix map: <http://mapping.example.com/> .

map:m_1 a rr:TriplesMap ;
    rml:logicalSource map:ls_1 ;
    rr:subjectMap map:s_1 ;
    rr:predicateObjectMap map:po_1 .
```

## 🐛 Troubleshooting

### Common Issues

1. **API Connection Failed**
   - Ensure the Rust backend is running on port 8080
   - Check CORS settings in the API

2. **Build Errors**
   - Clear node_modules and reinstall: `rm -rf node_modules && npm install`
   - Check Node.js version compatibility

3. **Test Failures**
   - Ensure all dependencies are installed
   - Check test environment configuration in `vitest.config.ts`

### Performance

- The app uses code splitting for optimal loading
- Vite provides fast HMR during development
- Production builds are optimized and minified

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `npm test`
5. Commit changes: `git commit -m 'Add amazing feature'`
6. Push to branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Development Guidelines

- Follow TypeScript best practices
- Write tests for new components
- Use existing UI components when possible
- Follow the established file structure
- Update documentation as needed

## 📄 License

This project is part of the ShExML2RML transpiler. See the main project LICENSE for details.

## 🔗 Links

- [ShExML Specification](https://shexml.herminiogarcia.com/spec/)
- [RML Specification](https://rml.io/specs/rml/)
- [Vite Documentation](https://vitejs.dev/)
- [React Documentation](https://react.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
