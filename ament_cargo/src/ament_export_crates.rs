// Copyright 2020 DCS Corporation, All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); 
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at //
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software 
// distributed under the License is distributed on an "AS IS" BASIS, 
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and 
// limitations under the License.
//
// DISTRIBUTION A. Approved for public release; distribution unlimited.
// OPSEC #4584.
//
// Delivered to the U.S. Government with Unlimited Rights, as defined in DFARS 
// Part 252.227-7013 or 7014 (Feb 2014).
//
// This notice must appear in all copies of this file and its derivatives.

use std::path::Path;

fn ament_register_extension(extension_point: &str, package_name: &str, filename: &Path) {
// #
// # Register a CMake filename to be included as part of an extension
// # point.
// #
// # :param extension_point: the name of the extension point
// # :type extension_point: string
// # :param package_name: the name of the package containing the CMake
// #   file
// # :type package_name: string
// # :param cmake_filename: the path to a CMake file relative to the
// #   ${package_name}_DIR folder
// # :type cmake_filename: string
// #
// # @public
// #
// macro(ament_register_extension extension_point package_name cmake_filename)
//   list(APPEND AMENT_EXTENSIONS_${extension_point}
//     "${package_name}:${cmake_filename}")
// endmacro()
}

fn register_package_hook() {
//     macro(_ament_cmake_export_crates_register_package_hook)
//     if(NOT DEFINED
//         _AMENT_CMAKE_EXPORT_CRATES_PACKAGE_HOOK_REGISTERED)
//       set(_AMENT_CMAKE_EXPORT_CRATES_PACKAGE_HOOK_REGISTERED TRUE)
  
//       find_package(ament_cmake_core QUIET REQUIRED)
//       ament_register_extension("ament_package"
//         "ament_cmake_export_crates"
//         "ament_cmake_export_crates_package_hook.cmake")
//     endif()
//   endmacro()
  
//   include(
//     "${ament_cmake_export_crates_DIR}/ament_export_crates.cmake")

}

fn ament_export_crates(crate_paths: &std::Vec<Path>) {
//     if(_${PROJECT_NAME}_AMENT_PACKAGE)
//       message(FATAL_ERROR
//         "ament_export_crates() must be called before ament_package()")
//     endif()
  
//     if(${ARGC} GREATER 0)
//       _ament_cmake_export_crates_register_package_hook()
  
//       foreach(_arg ${ARGN})
//         if(IS_ABSOLUTE "${_arg}")
//           if(NOT EXISTS "${_arg}")
//             message(WARNING
//               "ament_export_crates() package '${PROJECT_NAME}' exports the "
//               "crate '${_arg}' which doesn't exist")
//           else()
//               list_append_unique(_AMENT_EXPORT_ABSOLUTE_CRATES "${_arg}")
//           endif()
//         else()
//           set(_arg "\${${PROJECT_NAME}_DIR}/../../../${_arg}")
//           list_append_unique(_AMENT_EXPORT_RELATIVE_CRATES "${_arg}")
//         endif()
//       endforeach()
//     endif()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
