# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Keyboard shortcuts for undo (Ctrl+Z/Cmd+Z) and redo (Ctrl+Y/Cmd+Y/Ctrl+Shift+Z)
- Auto-save functionality to browser localStorage
- Auto-load from localStorage on editor initialization
- Clear button with confirmation modal to reset editor and clear saved state

## [0.1.0] - 2025-12-17

### Added

#### Core Features
- Initial release of yew-wysiwyg editor
- Core trait system for extensible widgets
- Widget registry for managing available widget types
- Theme system for customizable styling
- JSON serialization/deserialization for layouts

#### Standard Widgets
- Row container for horizontal layout
- Column container for vertical layout
- Grid container for responsive grid layout
- Text widget with rich formatting (bold, italic, underline)
- Heading widget (H1-H6)
- Paragraph widget

#### Editor Components
- Main Editor component with full editing interface
- Canvas component for rendering editable layouts
- Widget palette for browsing and adding widgets
- Toolbar with export/import functionality
- Widget selection and deletion

#### Developer Experience
- Comprehensive documentation and examples
- Demo application showcasing all features
- GitHub Actions CI/CD pipeline
- Automated testing and linting
- GitHub Pages deployment for demo
- WASM bundle size tracking

[Unreleased]: https://github.com/yourusername/yew-wysiwyg/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/yew-wysiwyg/releases/tag/v0.1.0
