function [A] = generate_A_matrix(N, sigma, r, B, K, gamma)
%generate_A_matrix Creates the NxN matrix A, which includes
%boundary conditions V(0, t) = 0, V(Smax, t) = Smax - Kexp(-...) is taken
%care of in the algo iteration itself
 S = generate_S(N, B, K);
 deltaS = S(2)-S(1);
 alpha = r*S/(2*deltaS);
 beta = sigma^2*S.^(2*gamma)/(2*deltaS^2);
 u = (-alpha-beta);
 u = u(1:end-1); %drop the last element
 d = r+2*beta;
 l = (alpha-beta);
 l = l(2:end); %drop the first element, and zero the last
 A = gallery('tridiag', l, d, u);
end
